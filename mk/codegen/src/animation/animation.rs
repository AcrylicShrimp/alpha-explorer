use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::{format_ident, quote, ToTokens, __private::TokenStream as QuoteTokenStream};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{
    parse_macro_input, Data, DeriveInput, Error as SynError, Ident, Index, LitStr,
    Result as SynResult, Token,
};

// TODO: Rework this.

#[derive(Clone)]
pub(crate) enum FieldRef {
    Ident(Ident),
    Index(Index),
}

impl ToTokens for FieldRef {
    fn to_tokens(&self, tokens: &mut QuoteTokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Index(index) => index.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AnimationFieldType {
    Bool,
    Integer,
    Float,
    String,
}

struct AnimationFieldArgument {
    pub field: LitStr,
    pub ty: AnimationFieldType,
}

impl Parse for AnimationFieldArgument {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut field = None;
        let mut ty = None;

        while !input.is_empty() {
            let arg_name = input.parse::<Ident>()?;

            if arg_name == "field" {
                if field.is_some() {
                    return Err(SynError::new(input.span(), "duplicated argument: field"));
                }

                input.parse::<Token![=]>()?;
                let parsed = input.parse::<LitStr>()?;
                field = Some(parsed);

                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                }
            } else if arg_name == "ty" {
                if ty.is_some() {
                    return Err(SynError::new(input.span(), "duplicated argument: ty"));
                }

                input.parse::<Token![=]>()?;
                let parsed = input.parse::<LitStr>()?;

                ty = Some(match parsed.value().as_str() {
                    "bool" => AnimationFieldType::Bool,
                    "integer" => AnimationFieldType::Integer,
                    "float" => AnimationFieldType::Float,
                    "string" => AnimationFieldType::String,
                    ty @ _ => {
                        return Err(SynError::new(
                            parsed.span(),
                            format!("invalid argument: type: {}", ty),
                        ));
                    }
                });

                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                }
            } else {
                return Err(SynError::new(
                    input.span(),
                    format!("invalid argument: {}", arg_name),
                ));
            }
        }

        let field = if let Some(field) = field {
            field
        } else {
            return Err(SynError::new(input.span(), "missing argument: field"));
        };

        let ty = if let Some(ty) = ty {
            ty
        } else {
            return Err(SynError::new(input.span(), "missing argument: ty"));
        };

        Ok(Self { field, ty })
    }
}

pub fn animation(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return TokenStream::new();
    };

    let name = input.ident;
    let name_snake = name.to_string().to_case(Case::Snake);
    let mut field_index = 0;
    let mut field_set = HashSet::new();
    let mut fields = vec![];
    let mut animation_fields = vec![];
    let mut matching_tys = vec![];
    let mut matching_as_tys = vec![];

    for field in &data.fields {
        for attr in &field.attrs {
            if let Some(ident) = attr.path.get_ident() {
                if ident == "animate" {
                    let argument = attr
                        .parse_args::<AnimationFieldArgument>()
                        .unwrap_or_abort();
                    let argument_field = argument.field.value();

                    if field_set.contains(&argument_field) {
                        emit_error!(
                            argument.field.span(),
                            "duplicated field: {} in the struct: {}",
                            argument_field,
                            name
                        );
                    } else {
                        field_set.insert(argument_field);
                    }

                    match &field.ident {
                        Some(ident) => {
                            fields.push(FieldRef::Ident(ident.clone()));
                        }
                        None => {
                            let index = Index::from(field_index);
                            field_index += 1;
                            fields.push(FieldRef::Index(index));
                        }
                    };

                    animation_fields.push(argument.field);
                    matching_tys.push(format_ident!(
                        "{}",
                        match &argument.ty {
                            AnimationFieldType::Bool => "bool",
                            AnimationFieldType::Integer => "i64",
                            AnimationFieldType::Float => "f64",
                            AnimationFieldType::String => "String",
                        }
                    ));
                    matching_as_tys.push(format_ident!(
                        "as_{}",
                        match &argument.ty {
                            AnimationFieldType::Bool => "bool",
                            AnimationFieldType::Integer => "integer",
                            AnimationFieldType::Float => "float",
                            AnimationFieldType::String => "string",
                        }
                    ));
                }
            }
        }
    }

    let expanded = quote! {
        impl crate::codegen_traits::Animate for #name {
            fn ty(&self) -> &'static str {
                #name_snake
            }

            fn animate(
                &mut self,
                time_line: &crate::animation::AnimationTimeLine,
                key_frame: &crate::animation::AnimationKeyFrame,
                normalized_time_in_key_frame: f32,
            ) {
                match time_line.field.as_str() {
                    #(
                        #animation_fields => {
                            self.#fields = <#matching_tys as crate::animation::Interpolatable>::interpolate(
                                key_frame.from.#matching_as_tys(),
                                key_frame.to.#matching_as_tys(),
                                normalized_time_in_key_frame,
                            ) as _;
                        }
                    )*
                    _ => {}
                }
            }
        }
    };

    TokenStream::from(expanded)
}

use crate::lua::impl_lua_method;
use proc_macro::TokenStream;
use proc_macro_error::ResultExt;
use quote::{__private::TokenStream as QuoteTokenStream, quote};
use syn::{
    parse::{Parse, ParseStream},
    token::Comma,
    DataStruct, DeriveInput, Ident, LitStr, Result as SynResult, Token, Type,
};

struct FieldName {
    pub name: LitStr,
}

impl Parse for FieldName {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(Self {
            name: input.parse()?,
        })
    }
}

struct UserType {
    pub ty: Type,
}

impl Parse for UserType {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(Self { ty: input.parse()? })
    }
}

struct UserFunc {
    pub getter: Option<Ident>,
    pub setter: Option<Ident>,
}

impl Parse for UserFunc {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut getter = None;
        let mut setter = None;

        while !input.is_empty() {
            let ty = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            let ident = input.parse::<Ident>()?;

            if input.peek(Comma) {
                input.parse::<Comma>()?;
            }

            if ty == "getter" {
                getter = Some(ident);
            } else if ty == "setter" {
                setter = Some(ident);
            }
        }

        Ok(Self { getter, setter })
    }
}

struct FieldGetter {
    pub lua_field_name: String,
    pub tokens: QuoteTokenStream,
}

struct FieldSetter {
    pub lua_field_name: String,
    pub tokens: QuoteTokenStream,
}

pub fn impl_lua_userdata(
    derive: &DeriveInput,
    input: &DataStruct,
    ty_name: &Ident,
    codegen_to_string: &mut dyn FnMut(&DataStruct) -> TokenStream,
    codegen_this_getter: Option<&mut dyn FnMut(&DataStruct) -> TokenStream>,
    codegen_this_setter: Option<&mut dyn FnMut(&DataStruct) -> TokenStream>,
) -> TokenStream {
    let ty_name_str = ty_name.to_string();
    let fields = input.fields.iter().filter(|&field| {
        !field.attrs.iter().any(|attr| {
            attr.path
                .get_ident()
                .map_or(false, |ident| ident == "lua_hidden")
        })
    });

    let mut field_getters = Vec::with_capacity(input.fields.len());
    let mut field_setters = Vec::with_capacity(input.fields.len());

    for field in fields {
        let field_name = if let Some(ident) = &field.ident {
            ident
        } else {
            continue;
        };

        let lua_field_name = field
            .attrs
            .iter()
            .find(|&attr| {
                attr.path
                    .get_ident()
                    .map_or(false, |ident| ident == "lua_field_name")
            })
            .map_or(field_name.to_string(), |attr| {
                let field_name = attr.parse_args::<FieldName>().unwrap_or_abort();
                field_name.name.value()
            });

        let lua_user_type = field
            .attrs
            .iter()
            .find(|&attr| {
                attr.path
                    .get_ident()
                    .map_or(false, |ident| ident == "lua_user_type")
            })
            .map(|attr| {
                let user_type = attr.parse_args::<UserType>().unwrap_or_abort();
                user_type.ty
            });

        let lua_user_func = field
            .attrs
            .iter()
            .find(|&attr| {
                attr.path
                    .get_ident()
                    .map_or(false, |ident| ident == "lua_user_func")
            })
            .map(|attr| attr.parse_args::<UserFunc>().unwrap_or_abort())
            .unwrap_or_else(|| UserFunc {
                getter: None,
                setter: None,
            });

        let lua_readonly = field.attrs.iter().any(|attr| {
            attr.path
                .get_ident()
                .map_or(false, |ident| ident == "lua_readonly")
        });

        if codegen_this_getter.is_some() {
            field_getters.push(FieldGetter {
                lua_field_name: lua_field_name.clone(),
                tokens: if let Some(getter) = lua_user_func.getter {
                    quote! {
                        this.#getter(lua)
                    }
                } else if let Some(ty) = &lua_user_type {
                    quote! {
                        <#ty>::from(this.#field_name.clone()).to_lua(lua)
                    }
                } else {
                    quote! {
                        this.#field_name.clone().to_lua(lua)
                    }
                },
            });
        }

        if codegen_this_setter.is_some() && !lua_readonly {
            let field_ty = &field.ty;
            field_setters.push(FieldSetter {
                lua_field_name: lua_field_name.clone(),
                tokens: if let Some(setter) = lua_user_func.setter {
                    quote! {
                        this.#setter(value, lua)?;
                    }
                } else if let Some(ty) = &lua_user_type {
                    quote! {
                        this.#field_name = <#field_ty>::from(<#ty as mlua::FromLua>::from_lua(value, lua)?);
                    }
                } else {
                    quote! {
                        this.#field_name = <#field_ty as mlua::FromLua>::from_lua(value, lua)?;
                    }
                },
            });
        }
    }

    let impl_to_string = QuoteTokenStream::from(codegen_to_string(input));
    let impl_this_getter = codegen_this_getter
        .map(|codegen_this_getter| QuoteTokenStream::from(codegen_this_getter(input)));
    let impl_this_setter = codegen_this_setter
        .map(|codegen_this_setter| QuoteTokenStream::from(codegen_this_setter(input)));

    let impl_metamethod_index = impl_this_getter.map(|impl_this_getter|{
        let mut lua_field_names = Vec::with_capacity(field_getters.len());
        let mut tokens = Vec::with_capacity(field_getters.len());

        for field_getter in field_getters {
            lua_field_names.push(field_getter.lua_field_name);
            tokens.push(field_getter.tokens);
        }

        quote! {
            methods.add_meta_method(
                mlua::MetaMethod::Index,
                |lua, this, index: String| -> mlua::Result<mlua::Value> {
                    match index.as_str() {
                        "_type" => #ty_name_str.to_lua(lua),
                        #(
                            #lua_field_names => {
                                #impl_this_getter
                                #tokens
                            }
                        )*
                        _ => Err(format!("the type {} has no such field '{}'", #ty_name_str, index).to_lua_err()),
                    }
                },
            );
        }
    })  .unwrap_or_else(|| quote! {});
    let impl_metamethod_newindex = impl_this_setter
        .map(|impl_this_setter| {
            let mut lua_field_names = Vec::with_capacity(field_setters.len());
            let mut tokens = Vec::with_capacity(field_setters.len());

            for field_setter in field_setters {
                lua_field_names.push(field_setter.lua_field_name);
                tokens.push(field_setter.tokens);
            }

            quote! {
                methods.add_meta_method_mut(
                    mlua::MetaMethod::NewIndex,
                    |lua, this, (index, value): (String, mlua::Value)| -> mlua::Result<()> {
                        match index.as_str() {
                            #(
                                #lua_field_names => {
                                    #impl_this_setter
                                    #tokens
                                    Ok(())
                                }
                            )*
                            _ => Err(format!("the type {} has no such field '{}'", #ty_name_str, index).to_lua_err()),
                        }
                    },
                );
            }
        })
        .unwrap_or_else(|| quote! {});

    let impl_lua_method = QuoteTokenStream::from(impl_lua_method(derive));

    TokenStream::from(quote! {
        impl mlua::UserData for #ty_name {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                use mlua::{ExternalError, ToLua};

                methods.add_meta_method(
                    mlua::MetaMethod::ToString,
                    |lua, this, ()| {
                        #impl_to_string
                    }
                );

                #impl_metamethod_index

                #impl_metamethod_newindex

                #impl_lua_method
            }
        }
    })
}

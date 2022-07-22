mod lua;

use convert_case::{Case, Casing};
use lua::lua_struct;
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::__private::TokenStream as QuoteTokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    parse_macro_input, Data, DeriveInput, Error as SynError, Ident, Index, LitStr,
    Result as SynResult, Token, Type,
};

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

#[proc_macro_derive(Animation, attributes(animate))]
#[proc_macro_error]
pub fn animation_derive(item: TokenStream) -> TokenStream {
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

struct UserFuncFieldArgument {
    pub get: Option<Ident>,
    pub set: Option<Ident>,
}

impl Parse for UserFuncFieldArgument {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut get = None;
        let mut set = None;

        while !input.is_empty() {
            let ty = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            let ident = input.parse::<Ident>()?;

            if input.peek(Comma) {
                input.parse::<Comma>()?;
            }

            if ty == "get" {
                get = Some(ident);
            } else if ty == "set" {
                set = Some(ident);
            } else {
                return Err(SynError::new(input.span(), "invalid argument"));
            }
        }

        Ok(Self { get, set })
    }
}

#[proc_macro_derive(
    LuaComponent,
    attributes(
        lua_userdata,
        lua_userfunc,
        lua_field,
        lua_hidden,
        lua_readonly,
        lua_method,
    )
)]
#[proc_macro_error]
pub fn lua_component_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let span = input.span();
    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return TokenStream::new();
    };

    let name = input.ident;
    let type_name = format!("component:{}", name);
    let wrapper_name = format_ident!("LuaComponent{}", name);
    let mut field_index = 0;
    let mut fields = vec![];
    let mut field_names = vec![];
    let mut non_readonly_fields = vec![];
    let mut non_readonly_field_names = vec![];
    let mut methods = vec![];
    let mut method_names = vec![];

    'field: for field in &data.fields {
        let mut userdata_type = None;
        let mut userfunc_name = None;
        let mut field_name = None;
        let mut readonly = false;
        let mut method = false;

        for attr in &field.attrs {
            if let Some(ident) = attr.path.get_ident() {
                if ident == "lua_method" {
                    method = true;
                    continue;
                }

                if method {
                    match &field.ident {
                        Some(field) => {
                            abort!(
                                attr.span(),
                                "the field {} is a method, it can't have other attributes",
                                field
                            );
                        }
                        None => {
                            abort!(
                                attr.span(),
                                "the field at index {} is a method, it can't have other attributes",
                                field_index
                            );
                        }
                    }
                }

                if ident == "lua_hidden" {
                    continue 'field;
                }

                if ident == "lua_userdata" {
                    userdata_type = Some(attr.parse_args::<Type>().unwrap_or_abort());
                } else if ident == "lua_userfunc" {
                    userfunc_name =
                        Some(attr.parse_args::<UserFuncFieldArgument>().unwrap_or_abort());
                } else if ident == "lua_field" {
                    field_name = Some(format_ident!(
                        "{}",
                        attr.parse_args::<LitStr>().unwrap_or_abort().value()
                    ));
                } else if ident == "lua_readonly" {
                    readonly = true;
                }
            }
        }

        let field_ref = match &field.ident {
            Some(ident) => FieldRef::Ident(ident.clone()),
            None => {
                let index = Index::from(field_index);
                field_index += 1;
                FieldRef::Index(index)
            }
        };
        let field_name = match field_name.as_ref() {
            Some(ident) => ident,
            None => match &field.ident {
                Some(ident) => ident,
                None => {
                    continue 'field;
                }
            },
        };

        if method {
            methods.push(field_name.clone());
            method_names.push(field_name.to_string());
        } else {
            if !readonly {
                let mut met_set_function = false;

                if let Some(userfunc_name) = &userfunc_name {
                    if let Some(func_name) = &userfunc_name.set {
                        met_set_function = true;
                        non_readonly_fields.push(quote! {
                            this.#func_name(value, lua)?;
                        });
                    }
                }

                if !met_set_function {
                    let field_ty = &field.ty;
                    non_readonly_fields.push(if let Some(userdata_type) = userdata_type.as_ref() {
                    quote! {
                        this.#field_ref = <#field_ty>::from(<#userdata_type as mlua::FromLua>::from_lua(value, lua)?);
                    }
                } else {
                    quote! {
                        this.#field_ref = <#field_ty as mlua::FromLua>::from_lua(value, lua)?;
                    }
                });
                }

                non_readonly_field_names.push(field_name.to_string());
            }

            let mut met_get_function = false;

            if let Some(userfunc_name) = &userfunc_name {
                if let Some(func_name) = &userfunc_name.get {
                    met_get_function = true;
                    fields.push(quote! {
                        this.#func_name(lua)
                    });
                }
            }

            if !met_get_function {
                fields.push(if let Some(userdata_type) = userdata_type.as_ref() {
                    quote! {
                        <#userdata_type>::from(this.#field_ref.clone()).to_lua(lua)
                    }
                } else {
                    quote! {
                        this.#field_ref.clone().to_lua(lua)
                    }
                });
            }

            field_names.push(field_name.to_string());
        }
    }

    if fields.is_empty() && methods.is_empty() {
        abort!(span, "the struct {} has no field or method to expose", name);
    }

    let field_impls = quote! {
        methods.add_meta_method(
            mlua::MetaMethod::Index,
            |lua, this, index: String| match index.as_str() {
                "_type" => #type_name.to_lua(lua),
                #(
                    #field_names => {
                        let mut world = crate::api::use_context().world_mut();
                        let entry = match world.entry(this.0) {
                            Some(entry) => entry,
                            None => return Ok(mlua::Value::Nil),
                        };
                        let this = match entry.get_component::<#name>() {
                            Ok(this) => this,
                            Err(_) => return Ok(mlua::Value::Nil),
                        };
                        #fields
                    }
                )*
                _ => Err(format!("the type {} has no such field '{}'", #type_name, index).to_lua_err()),
            },
        );
    };
    let non_readonly_field_impls = if non_readonly_fields.is_empty() {
        QuoteTokenStream::new()
    } else {
        quote! {
            methods.add_meta_method(
                mlua::MetaMethod::NewIndex,
                |lua, this, (index, value): (String, mlua::Value)| match index.as_str() {
                    #(
                        #non_readonly_field_names => {
                            let mut world = crate::api::use_context().world_mut();
                            let mut entry = match world.entry(this.0) {
                                Some(entry) => entry,
                                None => return Err(format!("the type {} used invalid entity id {:?}", #type_name, this.0).to_lua_err()),
                            };
                            let this = match entry.get_component_mut::<#name>() {
                                Ok(this) => this,
                                Err(_) => return Err(format!("the entity id {:?} does not contains the type {}", this.0, #type_name).to_lua_err()),
                            };

                            #non_readonly_fields
                            Ok(())
                        }
                    )*
                    _ => Err(format!("the type {} has no such field '{}'", #type_name, index).to_lua_err()),
                },
            );
        }
    };
    let method_impls = quote! {
        #(
            methods.add_method(#method_names, |lua, this, param: LuaMultiValue| {
                this.#methods(lua, <_>::from_lua_multi(param, lua)?)
            });
        )*
    };

    let expanded = quote! {
        impl<'world> std::convert::TryFrom<&'world legion::world::Entry<'world>> for &'world #name {
            type Error = legion::world::ComponentError;

            fn try_from(entry: &'world legion::world::Entry) -> Result<Self, Self::Error> {
                entry.get_component()
            }
        }

        impl<'world> std::convert::TryFrom<&'world mut legion::world::Entry<'world>> for &'world mut #name {
            type Error = legion::world::ComponentError;

            fn try_from(entry: &'world mut legion::world::Entry) -> Result<Self, Self::Error> {
                entry.get_component_mut()
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct #wrapper_name(pub legion::Entity);

        impl From<legion::Entity> for #wrapper_name {
            fn from(entity: legion::Entity) -> Self {
                Self(entity)
            }
        }

        impl From<#wrapper_name> for legion::Entity {
            fn from(component: #wrapper_name) -> Self {
                component.0
            }
        }

        impl mlua::UserData for #wrapper_name {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                use mlua::{ExternalError, ToLua};

                methods.add_meta_method(
                    mlua::MetaMethod::ToString,
                    |lua, this, ()| {
                        format!("{}{{entity id: {:?}}}", #type_name, this.0).to_lua(lua)
                    }
                );
                #field_impls
                #non_readonly_field_impls
                #method_impls
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(
    LuaComponentNoWrapper,
    attributes(
        lua_userdata,
        lua_userfunc,
        lua_field,
        lua_hidden,
        lua_readonly,
        lua_method,
    )
)]
#[proc_macro_error]
pub fn lua_component_no_wrapper_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let span = input.span();
    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return TokenStream::new();
    };

    let name = input.ident;
    let type_name = format!("component:{}", name);
    let mut field_index = 0;
    let mut fields = vec![];
    let mut field_names = vec![];
    let mut non_readonly_fields = vec![];
    let mut non_readonly_field_names = vec![];
    let mut methods = vec![];
    let mut method_names = vec![];

    'field: for field in &data.fields {
        let mut userdata_type = None;
        let mut userfunc_name = None;
        let mut field_name = None;
        let mut readonly = false;
        let mut method = false;

        for attr in &field.attrs {
            if let Some(ident) = attr.path.get_ident() {
                if ident == "lua_method" {
                    method = true;
                    continue;
                }

                if method {
                    match &field.ident {
                        Some(field) => {
                            abort!(
                                attr.span(),
                                "the field {} is a method, it can't have other attributes",
                                field
                            );
                        }
                        None => {
                            abort!(
                                attr.span(),
                                "the field at index {} is a method, it can't have other attributes",
                                field_index
                            );
                        }
                    }
                }

                if ident == "lua_hidden" {
                    continue 'field;
                }

                if ident == "lua_userdata" {
                    userdata_type = Some(attr.parse_args::<Type>().unwrap_or_abort());
                } else if ident == "lua_userfunc" {
                    userfunc_name =
                        Some(attr.parse_args::<UserFuncFieldArgument>().unwrap_or_abort());
                } else if ident == "lua_field" {
                    field_name = Some(format_ident!(
                        "{}",
                        attr.parse_args::<LitStr>().unwrap_or_abort().value()
                    ));
                } else if ident == "lua_readonly" {
                    readonly = true;
                } else if ident == "lua_method" {
                    method = true;
                }
            }
        }

        let field_ref = match &field.ident {
            Some(ident) => FieldRef::Ident(ident.clone()),
            None => {
                let index = Index::from(field_index);
                field_index += 1;
                FieldRef::Index(index)
            }
        };
        let field_name = match field_name.as_ref() {
            Some(ident) => ident,
            None => match &field.ident {
                Some(ident) => ident,
                None => {
                    continue 'field;
                }
            },
        };

        if method {
            methods.push(field_name.clone());
            method_names.push(field_name.to_string());
        } else {
            if !readonly {
                let mut met_set_function = false;

                if let Some(userfunc_name) = &userfunc_name {
                    if let Some(func_name) = &userfunc_name.set {
                        met_set_function = true;
                        non_readonly_fields.push(quote! {
                            this.#func_name(value, lua)?;
                        });
                    }
                }

                if !met_set_function {
                    let field_ty = &field.ty;
                    non_readonly_fields.push(if let Some(userdata_type) = userdata_type.as_ref() {
                        quote! {
                            this.#field_ref = <#field_ty>::from(<#userdata_type as mlua::FromLua>::from_lua(value, lua)?);
                        }
                    } else {
                        quote! {
                            this.#field_ref = <#field_ty as mlua::FromLua>::from_lua(value, lua)?;
                        }
                    });
                }

                non_readonly_field_names.push(field_name.to_string());
            }

            let mut met_get_function = false;

            if let Some(userfunc_name) = &userfunc_name {
                if let Some(func_name) = &userfunc_name.get {
                    met_get_function = true;
                    fields.push(quote! {
                        this.#func_name(lua)
                    });
                }
            }

            if !met_get_function {
                fields.push(if let Some(userdata_type) = userdata_type.as_ref() {
                    quote! {
                        <#userdata_type>::from(this.#field_ref.clone()).to_lua(lua)
                    }
                } else {
                    quote! {
                        this.#field_ref.clone().to_lua(lua)
                    }
                });
            }

            field_names.push(field_name.to_string());
        }
    }

    if fields.is_empty() && methods.is_empty() {
        abort!(span, "the struct {} has no field or method to expose", name);
    }

    let field_impls = quote! {
        methods.add_meta_method(
            mlua::MetaMethod::Index,
            |lua, this, index: String| match index.as_str() {
                "_type" => #type_name.to_lua(lua),
                #(
                    #field_names => {
                        #fields
                    }
                )*
                _ => Err(format!("the type {} has no such field '{}'", #type_name, index).to_lua_err()),
            },
        );
    };
    let non_readonly_field_impls = if non_readonly_fields.is_empty() {
        QuoteTokenStream::new()
    } else {
        quote! {
            methods.add_meta_method_mut(
                mlua::MetaMethod::NewIndex,
                |lua, this, (index, value): (String, mlua::Value)| match index.as_str() {
                    #(
                        #non_readonly_field_names => {
                            #non_readonly_fields
                            Ok(())
                        }
                    )*
                    _ => Err(format!("the type {} has no such field '{}'", #type_name, index).to_lua_err()),
                },
            );
        }
    };
    let method_impls = quote! {
        #(
            methods.add_method(#method_names, |lua, this, param: LuaMultiValue| {
                this.#methods(lua, <_>::from_lua_multi(param, lua)?)
            });
        )*
    };

    let expanded = quote! {
        impl<'world> std::convert::TryFrom<&'world legion::world::Entry<'world>> for &'world #name {
            type Error = legion::world::ComponentError;

            fn try_from(entry: &'world legion::world::Entry) -> Result<Self, Self::Error> {
                entry.get_component()
            }
        }

        impl<'world> std::convert::TryFrom<&'world mut legion::world::Entry<'world>> for &'world mut #name {
            type Error = legion::world::ComponentError;

            fn try_from(entry: &'world mut legion::world::Entry) -> Result<Self, Self::Error> {
                entry.get_component_mut()
            }
        }

        impl mlua::UserData for #name {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                use mlua::{ExternalError, ToLua};

                methods.add_meta_method(
                    mlua::MetaMethod::ToString,
                    |lua, this, ()| {
                        format!("{}{{{:?}}}", #type_name, this).to_lua(lua)
                    }
                );
                #field_impls
                #non_readonly_field_impls
                #method_impls
            }
        }
    };

    TokenStream::from(expanded)
}

struct GetOnlyUserFuncFieldArgument {
    pub get: Option<Ident>,
}

impl Parse for GetOnlyUserFuncFieldArgument {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut get = None;

        while !input.is_empty() {
            let ty = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;
            let ident = input.parse::<Ident>()?;

            if input.peek(Comma) {
                input.parse::<Comma>()?;
            }

            if ty == "get" {
                get = Some(ident);
            } else {
                return Err(SynError::new(input.span(), "invalid argument"));
            }
        }

        Ok(Self { get })
    }
}

#[proc_macro_derive(LuaRc, attributes(lua_userdata, lua_userfunc, lua_field, lua_hidden))]
#[proc_macro_error]
pub fn lua_rc_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let span = input.span();
    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return TokenStream::new();
    };

    let name = input.ident;
    let type_name = format!("rc:{}", name);
    let wrapper_name = format_ident!("LuaRc{}", name);
    let mut field_index = 0;
    let mut fields = vec![];
    let mut field_names = vec![];

    'field: for field in &data.fields {
        let mut userdata_type = None;
        let mut userfunc_name = None;
        let mut field_name = None;

        for attr in &field.attrs {
            if let Some(ident) = attr.path.get_ident() {
                if ident == "lua_hidden" {
                    continue 'field;
                }

                if ident == "lua_userdata" {
                    userdata_type = Some(attr.parse_args::<Type>().unwrap_or_abort());
                } else if ident == "lua_userfunc" {
                    userfunc_name = Some(
                        attr.parse_args::<GetOnlyUserFuncFieldArgument>()
                            .unwrap_or_abort(),
                    );
                } else if ident == "lua_field" {
                    field_name = Some(format_ident!(
                        "{}",
                        attr.parse_args::<LitStr>().unwrap_or_abort().value()
                    ));
                }
            }
        }

        let field_ref = &match &field.ident {
            Some(ident) => FieldRef::Ident(ident.clone()),
            None => {
                let index = Index::from(field_index);
                field_index += 1;
                FieldRef::Index(index)
            }
        };
        let field_name = match field_name.as_ref() {
            Some(ident) => ident,
            None => match &field.ident {
                Some(ident) => ident,
                None => {
                    continue 'field;
                }
            },
        };

        let mut met_get_function = false;

        if let Some(userfunc_name) = &userfunc_name {
            if let Some(func_name) = &userfunc_name.get {
                met_get_function = true;
                fields.push(quote! {
                    this.#func_name(lua)
                });
            }
        }

        if !met_get_function {
            fields.push(if let Some(userdata_type) = userdata_type.as_ref() {
                quote! {
                    <#userdata_type>::from(this.#field_ref.clone()).to_lua(lua)
                }
            } else {
                quote! {
                    this.#field_ref.clone().to_lua(lua)
                }
            });
        }

        field_names.push(field_name.to_string());
    }

    if fields.is_empty() {
        abort!(span, "the struct {} has no field to expose", name);
    }

    let expanded = quote! {
        #[derive(Clone)]
        pub struct #wrapper_name(pub std::sync::Arc<#name>);

        impl From<std::sync::Arc<#name>> for #wrapper_name {
            fn from(rc: std::sync::Arc<#name>) -> Self {
                Self(rc)
            }
        }

        impl From<#wrapper_name> for std::sync::Arc<#name> {
            fn from(rc: #wrapper_name) -> Self {
                rc.0
            }
        }

        impl mlua::UserData for #wrapper_name {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                use mlua::{ExternalError, ToLua};

                methods.add_meta_method(mlua::MetaMethod::ToString, |lua, this, ()| {
                    format!("{}{{ptr: {:p}}}", "rc:Sprite", std::sync::Arc::as_ptr(&this.0)).to_lua(lua)
                });
                methods.add_meta_method(
                    mlua::MetaMethod::Index,
                    |lua, this, index: String| match index.as_str() {
                        "_type" => #type_name.to_lua(lua),
                        #(
                            #field_names => {
                                let this = std::sync::Arc::<#name>::from(this.clone());
                                #fields
                            }
                        )*
                        _ => Err(format!("the type {} has no such field '{}'", #type_name, index).to_lua_err()),
                    },
                );
            }
        }
    };

    TokenStream::from(expanded)
}

struct LuaRcInput {
    pub ty: Type,
    pub name: Ident,
}

impl Parse for LuaRcInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let ty = input.parse()?;
        input.parse::<Token![as]>()?;
        let name = input.parse()?;

        Ok(Self { ty, name })
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn lua_rc(item: TokenStream) -> TokenStream {
    let LuaRcInput { ty, name } = parse_macro_input!(item as LuaRcInput);

    let type_name = format!("rc:{}", name);

    let expanded = quote! {
        #[derive(Clone)]
        pub struct #name(pub std::sync::Arc<#ty>);

        impl From<std::sync::Arc<#ty>> for #name {
            fn from(rc: std::sync::Arc<#ty>) -> Self {
                Self(rc)
            }
        }

        impl From<#name> for std::sync::Arc<#ty> {
            fn from(rc: #name) -> Self {
                rc.0
            }
        }

        impl mlua::UserData for #name {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                use mlua::{ExternalError, ToLua};

                methods.add_meta_method(mlua::MetaMethod::ToString, |lua, this, ()| {
                    format!("{}{{ptr: {:p}}}", "rc:Sprite", std::sync::Arc::as_ptr(&this.0)).to_lua(lua)
                });
                methods.add_meta_method(
                    mlua::MetaMethod::Index,
                    |lua, this, index: String| match index.as_str() {
                        "_type" => #type_name.to_lua(lua),
                        _ => Err(format!("the type {} has no such field '{}'", #type_name, index).to_lua_err()),
                    },
                );
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(LuaStruct)]
#[proc_macro_error]
pub fn _lua_struct(item: TokenStream) -> TokenStream {
    lua_struct(item)
}

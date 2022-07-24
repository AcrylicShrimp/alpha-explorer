use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, Result as SynResult, Token, Type,
};

struct Handle {
    pub ty: Type,
    pub name: Ident,
}

impl Parse for Handle {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let ty = input.parse()?;
        input.parse::<Token![as]>()?;
        let name = input.parse()?;

        Ok(Self { ty, name })
    }
}

pub fn lua_handle(item: TokenStream) -> TokenStream {
    let Handle { ty, name } = parse_macro_input!(item as Handle);
    let name_str = name.to_string();

    TokenStream::from(quote! {
        #[derive(Debug, Clone)]
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
                    format!("{}{{ptr: {:p}}}", #name_str, std::sync::Arc::as_ptr(&this.0)).to_lua(lua)
                });
                methods.add_meta_method(
                    mlua::MetaMethod::Index,
                    |lua, this, index: String| match index.as_str() {
                        "_type" => #name_str.to_lua(lua),
                        _ => Err(format!("the type {} has no such field '{}'", #name_str, index).to_lua_err()),
                    },
                );
            }
        }
    })
}

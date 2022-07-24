use crate::lua::impl_lua_userdata;
use proc_macro::TokenStream;
use quote::{__private::TokenStream as QuoteTokenStream, format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub fn lua_rc(item: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(item as DeriveInput);
    let input = if let Data::Struct(input) = &derive.data {
        input
    } else {
        return TokenStream::new();
    };

    let ty_name = &derive.ident;
    let wrapper_ty_name = format_ident!("LuaRc{}", derive.ident);
    let wrapper_ty_name_str = wrapper_ty_name.to_string();

    let impl_lua_userdata = QuoteTokenStream::from(impl_lua_userdata(
        &derive,
        input,
        &wrapper_ty_name,
        &mut |_| {
            TokenStream::from(quote! {
                format!(
                    "{}{{ptr: {:p}}}",
                    #wrapper_ty_name_str,
                    std::sync::Arc::as_ptr(&this.0)
                )
                .to_lua(lua)
            })
        },
        Some(&mut |_| {
            TokenStream::from(quote! {
                let this = std::sync::Arc::<#ty_name>::from(this.clone());
            })
        }),
        None,
    ));

    TokenStream::from(quote! {
        #[derive(Clone)]
        pub struct #wrapper_ty_name(pub std::sync::Arc<#ty_name>);

        impl From<std::sync::Arc<#ty_name>> for #wrapper_ty_name {
            fn from(rc: std::sync::Arc<#ty_name>) -> Self {
                Self(rc)
            }
        }

        impl From<#wrapper_ty_name> for std::sync::Arc<#ty_name> {
            fn from(rc: #wrapper_ty_name) -> Self {
                rc.0
            }
        }

        #impl_lua_userdata
    })
}

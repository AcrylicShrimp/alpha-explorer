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
                let this = this.inner();
            })
        }),
        Some(&mut |_| {
            TokenStream::from(quote! {
                let mut this = this.inner();
            })
        }),
    ));

    TokenStream::from(quote! {
        #[derive(Clone)]
        pub struct #wrapper_ty_name(pub std::sync::Arc<parking_lot::Mutex<#ty_name>>);

        impl #wrapper_ty_name {
            pub fn wrap(inner: #ty_name) -> Self {
                Self(std::sync::Arc::new(parking_lot::Mutex::new(inner)))
            }

            pub fn inner(&self) -> parking_lot::MutexGuard<#ty_name> {
                self.0.lock()
            }
        }

        impl From<#ty_name> for #wrapper_ty_name {
            fn from(inner: #ty_name) -> Self {
                Self::wrap(inner)
            }
        }

        impl From<std::sync::Arc<parking_lot::Mutex<#ty_name>>> for #wrapper_ty_name {
            fn from(rc: std::sync::Arc<parking_lot::Mutex<#ty_name>>) -> Self {
                Self(rc)
            }
        }

        impl From<#wrapper_ty_name> for std::sync::Arc<parking_lot::Mutex<#ty_name>> {
            fn from(rc: #wrapper_ty_name) -> Self {
                rc.0
            }
        }

        #impl_lua_userdata
    })
}

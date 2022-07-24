use crate::lua::impl_lua_userdata;
use proc_macro::TokenStream;
use quote::{__private::TokenStream as QuoteTokenStream, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub fn lua_component_no_wrapper(item: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(item as DeriveInput);
    let input = if let Data::Struct(input) = &derive.data {
        input
    } else {
        return TokenStream::new();
    };

    let ty_name = &derive.ident;
    let ty_name_str = ty_name.to_string();

    let impl_lua_userdata = QuoteTokenStream::from(impl_lua_userdata(
        &derive,
        input,
        &ty_name,
        &mut |_| {
            TokenStream::from(quote! {
                format!("{}{{{:?}}}", #ty_name_str, this).to_lua(lua)
            })
        },
        Some(&mut |_| TokenStream::from(quote! {})),
        Some(&mut |_| TokenStream::from(quote! {})),
    ));

    TokenStream::from(quote! {
        impl<'world> std::convert::TryFrom<&'world legion::world::Entry<'world>> for &'world #ty_name {
            type Error = legion::world::ComponentError;

            fn try_from(entry: &'world legion::world::Entry) -> Result<Self, Self::Error> {
                entry.get_component()
            }
        }

        impl<'world> std::convert::TryFrom<&'world mut legion::world::Entry<'world>> for &'world mut #ty_name {
            type Error = legion::world::ComponentError;

            fn try_from(entry: &'world mut legion::world::Entry) -> Result<Self, Self::Error> {
                entry.get_component_mut()
            }
        }

        #impl_lua_userdata
    })
}

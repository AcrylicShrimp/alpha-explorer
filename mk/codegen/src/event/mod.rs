use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn event(item: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(item as DeriveInput);
    let input = if let Data::Struct(input) = &derive.data {
        input
    } else {
        return TokenStream::new();
    };

    let ty_name = &derive.ident;
    let ty_name_str = ty_name.to_string();
    let mut field_impls = Vec::new();

    for field in &input.fields {
        if let Some(ident) = &field.ident {
            let ident_str = ident.to_string();
            field_impls.push(quote! {
                #ident_str => Some(&self.#ident),
            });
        }
    }

    TokenStream::from(quote! {
        impl crate::event::Event for #ty_name {
            fn type_id() -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }

            fn name() -> &'static str {
                #ty_name_str
            }

            fn param(&self, param_name: &str) -> Option<&dyn std::any::Any> {
                match param_name {
                    #(#field_impls)*
                    _ => None,
                }
            }
        }
    })
}

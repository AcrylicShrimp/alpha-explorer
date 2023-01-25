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
    let mut event_field_impls = Vec::new();
    let mut lua_event_field_impls = Vec::new();

    for field in &input.fields {
        if let Some(ident) = &field.ident {
            let ident_str = ident.to_string();
            event_field_impls.push(quote! {
                #ident_str => Some(&self.#ident),
            });
            lua_event_field_impls.push(quote! {
                table.set(#ident_str, self.#ident.clone())?;
            });
        }
    }

    TokenStream::from(quote! {
        impl crate::event::Event for #ty_name {
            fn name(&self) -> &str {
                #ty_name_str
            }

            fn param(&self, param_name: &str) -> Option<&dyn std::any::Any> {
                match param_name {
                    #(#event_field_impls)*
                    _ => None,
                }
            }
        }

        impl crate::event::ParamsToLuaTable for #ty_name {
            fn params_to_lua_table<'lua>(&self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Table<'lua>> {
                let table = lua.create_table()?;
                #(#lua_event_field_impls)*
                Ok(table)
            }
        }
    })
}

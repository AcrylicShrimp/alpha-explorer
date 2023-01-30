use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn lua_enum(item: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(item as DeriveInput);
    let input = if let Data::Enum(input) = &derive.data {
        input
    } else {
        return TokenStream::new();
    };

    let ty_name = &derive.ident;
    let mut to_string_impls = Vec::new();
    let mut api_table_impls = Vec::new();

    for variant in &input.variants {
        let ident = &variant.ident;
        to_string_impls.push(quote! {
            #ty_name::#ident => Ok(stringify!(#ident)),
        });
        api_table_impls.push(quote! {
            table.set(stringify!(#ident), #ty_name::#ident)?;
        });
    }

    TokenStream::from(quote! {
        impl mlua::UserData for #ty_name {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                methods.add_meta_method(mlua::MetaMethod::ToString, |_lua, this, ()| match this {
                    #(#to_string_impls)*
                });
                methods.add_meta_function(mlua::MetaMethod::Eq, |_lua, (lhs, rhs): (Self, Self)| {
                    Ok(lhs == rhs)
                });
            }
        }

        impl crate::script::LuaApiTable for #ty_name {
            fn create_api_table<'lua>(lua: &'lua mlua::Lua) -> mlua::Result<mlua::Table<'lua>> {
                let table = lua.create_table()?;
                #(#api_table_impls)*
                Ok(table)
            }
        }
    })
}

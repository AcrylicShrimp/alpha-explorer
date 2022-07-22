use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn lua_struct(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let data = if let Data::Struct(data) = input.data {
        data
    } else {
        return TokenStream::new();
    };

    let name = input.ident;
    let name_str = name.to_string();
    let from_lua_field_expanded = data.fields.iter().map(|field| {
        let field_name = field
            .ident
            .as_ref()
            .expect_or_abort("field name is missing");
        let field_name_str = field_name.to_string();
        quote! {
            #field_name: value.get(#field_name_str)?,
        }
    });
    let to_lua_field_expanded = data.fields.iter().map(|field| {
        let field_name = field
            .ident
            .as_ref()
            .expect_or_abort("field name is missing");
        let field_name_str = field_name.to_string();
        quote! {
            (#field_name_str, self.#field_name.to_lua(lua)?),
        }
    });
    let expanded = quote! {
        impl<'lua> mlua::FromLua<'lua> for #name {
            fn from_lua(value: mlua::Value<'lua>, _: &'lua mlua::Lua) -> mlua::Result<Self> {
                use mlua::ExternalError;

                match value {
                    mlua::Value::Table(value) => Ok(Self {
                        #(#from_lua_field_expanded)*
                    }),
                    _ => {
                        return Err(format!("the type {} must be a table", #name_str).to_lua_err());
                    }
                }
            }
        }

        impl<'lua> mlua::ToLua<'lua> for #name {
            fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table(
                    lua.create_table_from([
                        #(#to_lua_field_expanded)*
                    ])?,
                ))
            }
        }
    };

    TokenStream::from(expanded)
}

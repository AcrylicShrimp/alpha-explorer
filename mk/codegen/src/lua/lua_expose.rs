use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Ident};

// TODO: Provide a way to implement ApiTable somehow. Maybe we need to introduce 1-depth namespaces.

pub fn lua_expose(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match input.data {
        Data::Struct(data) => lua_expose_struct(input.ident, input.attrs, data),
        Data::Enum(data) => lua_expose_enum(input.ident, input.attrs, data),
        _ => TokenStream::new(),
    }
}

macro_rules! has_attr {
    ($ident:literal in $attrs:expr) => {
        $attrs
            .iter()
            .any(|attr| attr.path.get_ident().map_or(false, |ident| ident == $ident))
    };
}

macro_rules! filter_attr {
    ($item:expr, $ident:literal) => {
        if $item
            .attrs
            .iter()
            .any(|attr| attr.path.get_ident().map_or(false, |ident| ident == $ident))
        {
            return None;
        }
    };
}

macro_rules! get_ident {
    ($item:expr) => {
        if let Some(ident) = &$item.ident {
            ident.clone()
        } else {
            return None;
        }
    };
}

fn lua_expose_struct(ident: Ident, attrs: Vec<Attribute>, data: DataStruct) -> TokenStream {
    let ident_str = ident.to_string();
    let from_lua_fields = data.fields.iter().filter_map(|field| {
        filter_attr!(field, "hidden");

        let field_name = get_ident!(field);
        let field_name_str = field_name.to_string();

        Some(quote! {
            #field_name: value.get(#field_name_str)?,
        })
    });
    let to_lua_fields = data.fields.iter().filter_map(|field| {
        filter_attr!(field, "hidden");

        let field_name = get_ident!(field);
        let field_name_str = field_name.to_string();

        Some(quote! {
            (#field_name_str, self.#field_name.to_lua(lua)?),
        })
    });

    let from_lua_impls = if !has_attr!("no_from_lua" in attrs) {
        quote! {
            impl<'lua> mlua::FromLua<'lua> for #ident {
                fn from_lua(value: mlua::Value<'lua>, _: &'lua mlua::Lua) -> mlua::Result<Self> {
                    use mlua::ExternalError;

                    match value {
                        mlua::Value::Table(value) => Ok(Self {
                            #(#from_lua_fields)*
                        }),
                        _ => {
                            return Err(format!("the type {} must be a table", #ident_str).to_lua_err());
                        }
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    let to_lua_impls = quote! {
        impl<'lua> mlua::ToLua<'lua> for #ident {
            fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(mlua::Value::Table(
                    lua.create_table_from([
                        #(#to_lua_fields)*
                    ])?,
                ))
            }
        }
    };

    TokenStream::from(quote! {
        #from_lua_impls
        #to_lua_impls
    })
}

fn lua_expose_enum(ident: Ident, _attrs: Vec<Attribute>, data: DataEnum) -> TokenStream {
    let ident_str = ident.to_string();
    let from_lua_variants = data.variants.iter().filter_map(|variant| {
        filter_attr!(variant, "hidden");

        let variant_name = variant.ident.clone();
        let variant_name_str = variant_name.to_string();

        Some(quote! {
            #variant_name_str => Self::#variant_name,
        })
    });
    let to_lua_variants = data.variants.iter().filter_map(|variant| {
        filter_attr!(variant, "hidden");

        let variant_name = variant.ident.clone();
        let variant_name_str = variant_name.to_string();

        Some(quote! {
            Self::#variant_name => #variant_name_str.to_lua(lua),
        })
    });

    let from_lua_impls = quote! {
        impl<'lua> mlua::FromLua<'lua> for #ident {
            fn from_lua(value: mlua::Value<'lua>, _: &'lua mlua::Lua) -> mlua::Result<Self> {
                use mlua::ExternalError;

                match value {
                    mlua::Value::String(str) => Ok(match str.to_str(lua)? {
                        #(#from_lua_variants)*
                        str => {
                            return Err(format!("the given string \"{}\" is not valid value for the type {}", str, #ident_str).to_lua_err());
                        }
                    }),
                    _ => {
                        return Err(format!("the type {} must be a string", #ident_str).to_lua_err());
                    }
                }
            }
        }
    };
    let to_lua_impls = quote! {
        impl<'lua> mlua::ToLua<'lua> for #ident {
            fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                Ok(match self {
                    #(#to_lua_variants)*
                    _ => mlua::Value::Nil,
                })
            }
        }
    };

    TokenStream::from(quote! {
        #from_lua_impls
        #to_lua_impls
    })
}

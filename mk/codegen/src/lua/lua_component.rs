use crate::lua::impl_lua_userdata;
use proc_macro::TokenStream;
use quote::{__private::TokenStream as QuoteTokenStream, format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

pub fn lua_component(item: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(item as DeriveInput);
    let input = if let Data::Struct(input) = &derive.data {
        input
    } else {
        return TokenStream::new();
    };

    let ty_name = &derive.ident;
    let wrapper_ty_name = format_ident!("LuaComponent{}", derive.ident);
    let wrapper_ty_name_str = wrapper_ty_name.to_string();

    let impl_lua_userdata = QuoteTokenStream::from(impl_lua_userdata(
        &derive,
        input,
        &wrapper_ty_name,
        &mut |_| {
            TokenStream::from(quote! {
                format!("{}{{entity id={:?}}}", #wrapper_ty_name_str, this.0).to_lua(lua)
            })
        },
        Some(&mut |_| {
            TokenStream::from(quote! {
                let mut world = crate::api::use_context().world_mut();
                let entry = match world.entry(this.0) {
                    Some(entry) => entry,
                    None => return Ok(mlua::Value::Nil),
                };
                let this = match entry.get_component::<#ty_name>() {
                    Ok(this) => this,
                    Err(_) => return Ok(mlua::Value::Nil),
                };
            })
        }),
        Some(&mut |_| {
            TokenStream::from(quote! {
                let mut world = crate::api::use_context().world_mut();
                let mut entry = match world.entry(this.0) {
                    Some(entry) => entry,
                    None => return Err(format!("invalid entity id {:?} was referenced by {}", this.0, #wrapper_ty_name_str).to_lua_err()),
                };
                let mut this = match entry.get_component_mut::<#ty_name>() {
                    Ok(this) => this,
                    Err(_) => return Err(format!("the entity id {:?} does not have {}", this.0, #wrapper_ty_name_str).to_lua_err()),
                };
            })
        }),
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

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub struct #wrapper_ty_name(pub legion::Entity);

        impl From<legion::Entity> for #wrapper_ty_name {
            fn from(entity: legion::Entity) -> Self {
                Self(entity)
            }
        }

        impl From<#wrapper_ty_name> for legion::Entity {
            fn from(component: #wrapper_ty_name) -> Self {
                component.0
            }
        }

        #impl_lua_userdata
    })
}

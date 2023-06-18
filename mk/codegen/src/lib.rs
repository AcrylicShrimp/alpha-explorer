mod animation;
mod event;
mod lua;

use animation::animation;
use event::event;
use lua::{
    lua_component, lua_component_no_wrapper, lua_enum, lua_expose, lua_handle, lua_rc, lua_struct,
    lua_user_data,
};
use proc_macro::TokenStream;
use proc_macro_error::*;

#[proc_macro_derive(Animation, attributes(animate))]
#[proc_macro_error]
pub fn _animation(item: TokenStream) -> TokenStream {
    animation(item)
}

#[proc_macro_derive(Event, attributes(event_name))]
#[proc_macro_error]
pub fn _event(item: TokenStream) -> TokenStream {
    event(item)
}

#[proc_macro_derive(
    LuaComponent,
    attributes(
        lua_name,
        lua_method,
        lua_hidden,
        lua_readonly,
        lua_field_name,
        lua_user_type,
        lua_user_func,
    )
)]
#[proc_macro_error]
pub fn _lua_component(item: TokenStream) -> TokenStream {
    lua_component(item)
}

#[proc_macro_derive(
    LuaComponentNoWrapper,
    attributes(
        lua_name,
        lua_method,
        lua_hidden,
        lua_readonly,
        lua_field_name,
        lua_user_type,
        lua_user_func,
    )
)]
#[proc_macro_error]
pub fn _lua_component_no_wrapper(item: TokenStream) -> TokenStream {
    lua_component_no_wrapper(item)
}

#[proc_macro_derive(LuaEnum)]
#[proc_macro_error]
pub fn _lua_enum(item: TokenStream) -> TokenStream {
    lua_enum(item)
}

#[proc_macro]
#[proc_macro_error]
pub fn define_lua_handle(item: TokenStream) -> TokenStream {
    lua_handle(item)
}

#[proc_macro_derive(
    LuaRc,
    attributes(
        lua_name,
        lua_method,
        lua_hidden,
        lua_readonly,
        lua_field_name,
        lua_user_type,
        lua_user_func,
    )
)]
#[proc_macro_error]
pub fn _lua_rc(item: TokenStream) -> TokenStream {
    lua_rc(item)
}

#[proc_macro_derive(LuaStruct)]
#[proc_macro_error]
pub fn _lua_struct(item: TokenStream) -> TokenStream {
    lua_struct(item)
}

#[proc_macro_derive(LuaExpose, attributes(no_from_lua, hidden))]
#[proc_macro_error]
pub fn _lua_expose(item: TokenStream) -> TokenStream {
    lua_expose(item)
}

#[proc_macro_derive(
    LuaUserData,
    attributes(impl_copy, readonly, hidden, rename, use_getter, use_setter)
)]
#[proc_macro_error]
pub fn _lua_user_data(item: TokenStream) -> TokenStream {
    lua_user_data(item)
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn lua_user_data_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    lua::lua_user_data_method(attr, item)
}

#[proc_macro_attribute]
pub fn hidden(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn rename(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn no_except(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn ops_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn ops_extra(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

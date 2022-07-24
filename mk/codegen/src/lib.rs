mod animation;
mod lua;

use animation::animation;
use lua::{lua_component, lua_component_no_wrapper, lua_handle, lua_rc, lua_struct};
use proc_macro::TokenStream;
use proc_macro_error::*;

#[proc_macro_derive(Animation, attributes(animate))]
#[proc_macro_error]
pub fn _animation(item: TokenStream) -> TokenStream {
    animation(item)
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

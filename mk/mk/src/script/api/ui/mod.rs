use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod ui_anchor;
mod ui_image_render_mode;
mod ui_margin;
mod ui_scale_mode;

pub use ui_anchor::*;
pub use ui_image_render_mode::*;
pub use ui_margin::*;
pub use ui_scale_mode::*;

pub struct UIModule;

impl LuaApiTable for UIModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("Anchor", ui_anchor::UIAnchor::create_api_table(lua)?)?;
        table.set(
            "ImageRenderMode",
            ui_image_render_mode::UIImageRenderMode::create_api_table(lua)?,
        )?;
        table.set("Margin", ui_margin::UIMargin::create_api_table(lua)?)?;
        table.set(
            "ScaleMode",
            ui_scale_mode::UIScaleMode::create_api_table(lua)?,
        )?;

        Ok(table)
    }
}

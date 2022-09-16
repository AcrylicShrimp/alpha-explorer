use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod glyph_layout_config;
mod horizontal_align;
mod vertical_align;
mod wrap_style;

pub use glyph_layout_config::*;
pub use horizontal_align::*;
pub use vertical_align::*;
pub use wrap_style::*;

pub struct GlyphModule;

impl LuaApiTable for GlyphModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "GlyphLayoutConfig",
            glyph_layout_config::GlyphLayoutConfig::create_api_table(lua)?,
        )?;
        table.set(
            "HorizontalAlign",
            horizontal_align::HorizontalAlign::create_api_table(lua)?,
        )?;
        table.set(
            "VerticalAlign",
            vertical_align::VerticalAlign::create_api_table(lua)?,
        )?;
        table.set("WrapStyle", wrap_style::WrapStyle::create_api_table(lua)?)?;

        Ok(table)
    }
}

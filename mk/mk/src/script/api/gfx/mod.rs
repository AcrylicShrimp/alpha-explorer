use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod clear_mode;
mod color;
mod font;
mod glyph_layout_config;
mod horizontal_align;
mod layer;
mod shader;
mod sprite;
mod sprite_slice;
mod sprite_texel_mapping;
mod texture;
mod vertical_align;
mod wrap_style;

pub use clear_mode::*;
pub use color::*;
pub use font::*;
pub use glyph_layout_config::*;
pub use horizontal_align::*;
pub use layer::*;
pub use shader::*;
pub use sprite::*;
pub use sprite_slice::*;
pub use texture::*;
pub use vertical_align::*;
pub use wrap_style::*;

pub struct GfxModule;

impl LuaApiTable for GfxModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        // table.set("AlphaTile", alpha_tile::AlphaTile::create_api_table(lua)?)?;
        // table.set(
        //     "AlphaTilemap",
        //     alpha_tilemap::AlphaTilemap::create_api_table(lua)?,
        // )?;
        // table.set(
        //     "AlphaTileset",
        //     alpha_tileset::AlphaTileset::create_api_table(lua)?,
        // )?;
        table.set("ClearMode", clear_mode::ClearMode::create_api_table(lua)?)?;
        table.set("Color", color::Color::create_api_table(lua)?)?;
        table.set(
            "GlyphLayoutConfig",
            glyph_layout_config::GlyphLayoutConfig::create_api_table(lua)?,
        )?;
        table.set(
            "HorizontalAlign",
            horizontal_align::HorizontalAlign::create_api_table(lua)?,
        )?;
        table.set("Layer", layer::Layer::create_api_table(lua)?)?;
        table.set(
            "VerticalAlign",
            vertical_align::VerticalAlign::create_api_table(lua)?,
        )?;
        table.set("WrapStyle", wrap_style::WrapStyle::create_api_table(lua)?)?;

        Ok(table)
    }
}

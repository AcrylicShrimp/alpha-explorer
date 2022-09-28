use crate::script::api::LuaApiTable;
use mlua::prelude::*;

mod alpha_tile;
mod alpha_tilemap;
mod alpha_tileset;
mod buffer;
mod clear_mode;
mod color;
mod font;
mod layer;
mod shader;
mod sprite;
mod sprite_atlas;
mod sprite_atlas_grid;
mod sprite_nine_patch;
mod texture;
mod tilemap;

pub use alpha_tile::*;
pub use alpha_tilemap::*;
pub use alpha_tileset::*;
pub use buffer::*;
pub use clear_mode::*;
pub use color::*;
pub use font::*;
pub use layer::*;
pub use shader::*;
pub use sprite::*;
pub use sprite_atlas::*;
pub use sprite_atlas_grid::*;
pub use sprite_nine_patch::*;
pub use texture::*;
pub use tilemap::*;

pub struct RenderModule;

impl LuaApiTable for RenderModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("AlphaTile", alpha_tile::AlphaTile::create_api_table(lua)?)?;
        table.set(
            "AlphaTilemap",
            alpha_tilemap::AlphaTilemap::create_api_table(lua)?,
        )?;
        table.set(
            "AlphaTileset",
            alpha_tileset::AlphaTileset::create_api_table(lua)?,
        )?;
        table.set("ClearMode", clear_mode::ClearMode::create_api_table(lua)?)?;
        table.set("Color", color::Color::create_api_table(lua)?)?;
        table.set("Layer", layer::Layer::create_api_table(lua)?)?;
        table.set(
            "SpriteChannel",
            crate::render::SpriteChannel::create_api_table(lua)?,
        )?;
        table.set(
            "TexelMapping",
            crate::render::TexelMapping::create_api_table(lua)?,
        )?;
        table.set("Tilemap", tilemap::Tilemap::create_api_table(lua)?)?;

        Ok(table)
    }
}

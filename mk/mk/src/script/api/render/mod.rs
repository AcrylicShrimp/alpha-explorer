use crate::script::api::ModuleType;
use rhai::Module;

mod alpha_tile;
mod alpha_tilemap;
mod alpha_tileset;
mod buffer;
mod color;
mod font;
mod layer;
mod shader;
mod sprite;
mod sprite_atlas;
mod sprite_atlas_grid;
mod sprite_nine_patch;
mod texel_mapping;
mod texture;
mod tilemap;

pub use alpha_tile::*;
pub use alpha_tilemap::*;
pub use alpha_tileset::*;
pub use buffer::*;
pub use color::*;
pub use font::*;
pub use layer::*;
pub use shader::*;
pub use sprite::*;
pub use sprite_atlas::*;
pub use sprite_atlas_grid::*;
pub use sprite_nine_patch::*;
pub use texel_mapping::*;
pub use texture::*;
pub use tilemap::*;

pub struct RenderModule;

impl ModuleType for RenderModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        alpha_tile::AlphaTile::register(&mut sub_module);
        alpha_tilemap::AlphaTilemap::register(&mut sub_module);
        alpha_tileset::AlphaTileset::register(&mut sub_module);
        buffer::Buffer::register(&mut sub_module);
        color::Color::register(&mut sub_module);
        font::Font::register(&mut sub_module);
        layer::Layer::register(&mut sub_module);
        shader::Shader::register(&mut sub_module);
        sprite::Sprite::register(&mut sub_module);
        sprite_atlas::SpriteAtlas::register(&mut sub_module);
        sprite_atlas_grid::SpriteAtlasGrid::register(&mut sub_module);
        sprite_nine_patch::SpriteNinePatch::register(&mut sub_module);
        texel_mapping::TexelMapping::register(&mut sub_module);
        texture::Texture::register(&mut sub_module);
        tilemap::Tilemap::register(&mut sub_module);

        module.set_sub_module("render", sub_module);
    }
}

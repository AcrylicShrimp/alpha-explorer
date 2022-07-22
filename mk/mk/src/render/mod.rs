mod color;
// mod glyph_manager;
// mod glyph_texture;
mod layer;
mod render_manager;
mod screen_manager;
mod sprite;
mod sprite_atlas;
mod sprite_atlas_grid;
mod sprite_nine_patch;
mod tilemap;

pub use color::*;
// pub use glyph_manager::*;
// pub use glyph_texture::*;
pub use layer::*;
pub use render::*;
pub use render_manager::*;
pub use screen_manager::*;
pub use sprite::*;
pub use sprite_atlas::*;
pub use sprite_atlas_grid::*;
pub use sprite_nine_patch::*;
pub use tilemap::*;

use codegen::lua_rc;
use fontdue::Font;

lua_rc!(Font as LuaRcFont);
lua_rc!(Buffer as LuaRcBuffer);
lua_rc!(Shader as LuaRcShader);
lua_rc!(Texture as LuaRcTexture);

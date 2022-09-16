use mlua::prelude::*;

pub trait EntityBuilderParam
where
    Self: Sized,
{
    fn from_table<'lua>(table: LuaTable<'lua>) -> LuaResult<Self>;
}

mod alpha_tilemap_renderer;
mod audio_source;
mod camera_params;
mod glyph_renderer;
mod nine_patch_renderer;
mod sprite_renderer;
mod tilemap_renderer;
mod ui_element;
mod ui_scaler;

pub use alpha_tilemap_renderer::*;
pub use audio_source::*;
pub use camera_params::*;
pub use glyph_renderer::*;
pub use nine_patch_renderer::*;
pub use sprite_renderer::*;
pub use tilemap_renderer::*;
pub use ui_element::*;
pub use ui_scaler::*;

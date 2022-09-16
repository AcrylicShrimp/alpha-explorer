use specs::prelude::*;

mod alpha_tilemap_renderer;
mod audio_source;
mod camera;
mod diagnostic;
mod glyph_renderer;
mod nine_patch_renderer;
mod single_animator;
mod size;
mod sprite_renderer;
mod tilemap_renderer;
mod transform;
mod ui_element;
mod ui_scaler;

pub use alpha_tilemap_renderer::*;
pub use audio_source::*;
pub use camera::*;
pub use diagnostic::*;
pub use glyph_renderer::*;
pub use nine_patch_renderer::*;
pub use single_animator::*;
pub use size::*;
pub use sprite_renderer::*;
pub use tilemap_renderer::*;
pub use transform::*;
pub use ui_element::*;
pub use ui_scaler::*;

pub fn register_components(world: &mut World) {
    world.register::<alpha_tilemap_renderer::AlphaTilemapRenderer>();
    world.register::<audio_source::AudioSource>();
    world.register::<camera::Camera>();
    world.register::<diagnostic::Diagnostic>();
    world.register::<glyph_renderer::GlyphRenderer>();
    world.register::<nine_patch_renderer::NinePatchRenderer>();
    // world.register::<single_animator::SingleAnimator>();
    world.register::<size::Size>();
    world.register::<sprite_renderer::SpriteRenderer>();
    world.register::<tilemap_renderer::TilemapRenderer>();
    world.register::<transform::Transform>();
    world.register::<ui_element::UIElement>();
    world.register::<ui_scaler::UIScaler>();
}

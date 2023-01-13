use specs::prelude::*;

// mod alpha_tilemap_renderer;
mod audio_source;
mod camera;
mod diagnostic;
mod glyph_renderer;
// mod single_animator;
mod size;
mod sprite_renderer;
// mod tilemap_renderer;
mod transform;
mod ui_element;
mod ui_mask;
mod ui_scaler;

// pub use alpha_tilemap_renderer::*;
pub use audio_source::*;
pub use camera::*;
pub use diagnostic::*;
pub use glyph_renderer::*;
// pub use single_animator::*;
pub use size::*;
pub use sprite_renderer::*;
// pub use tilemap_renderer::*;
pub use transform::*;
pub use ui_element::*;
pub use ui_mask::*;
pub use ui_scaler::*;

pub fn register_components(world: &mut World) {
    // world.register::<AlphaTilemapRenderer>();
    world.register::<AudioSource>();
    world.register::<Camera>();
    world.register::<Diagnostic>();
    world.register::<GlyphRenderer>();
    // world.register::<SingleAnimator>();
    world.register::<Size>();
    world.register::<SpriteRenderer>();
    // world.register::<TilemapRenderer>();
    world.register::<Transform>();
    world.register::<UIElement>();
    world.register::<UIMask>();
    world.register::<UIScaler>();
}

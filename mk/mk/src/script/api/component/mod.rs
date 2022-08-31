use crate::{engine::use_context, script::api::ModuleType};
use legion::{world::EntityAccessError, Entity, EntityStore};
use rhai::Module;
use std::{
    any::type_name,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct Component<T>
where
    T: 'static + Send + Sync,
{
    entity: Entity,
    _marker: PhantomData<T>,
}

impl<T> Component<T>
where
    T: 'static + Send + Sync,
{
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            _marker: PhantomData,
        }
    }

    pub fn entity(self) -> Entity {
        self.entity
    }

    pub fn is_exists(&self) -> bool {
        self.with_ref(|_| true).unwrap_or_default()
    }

    pub fn with_ref<R>(self, f: impl FnOnce(&T) -> R) -> Option<R> {
        let world = use_context().world();
        let entry = match world.entry_ref(self.entity) {
            Ok(entry) => entry,
            Err(err) => match err {
                EntityAccessError::AccessDenied => panic!(
                    "failed to access ref of component type '{}' of entity {:?}",
                    type_name::<T>(),
                    self.entity
                ),
                EntityAccessError::EntityNotFound => return None,
            },
        };
        Some(f(entry.get_component::<T>().unwrap()))
    }

    pub fn with_mut<R>(self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        let mut world = use_context().world_mut();
        let mut entry = match world.entry_mut(self.entity) {
            Ok(entry) => entry,
            Err(err) => match err {
                EntityAccessError::AccessDenied => panic!(
                    "failed to access mut ref of component type '{}' of entity {:?}",
                    type_name::<T>(),
                    self.entity
                ),
                EntityAccessError::EntityNotFound => return None,
            },
        };
        Some(f(entry.get_component_mut::<T>().unwrap()))
    }
}

impl<T> Clone for Component<T>
where
    T: 'static + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Copy for Component<T> where T: 'static + Send + Sync {}

impl<T> PartialEq for Component<T>
where
    T: 'static + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl<T> Eq for Component<T> where T: 'static + Send + Sync {}

impl<T> Hash for Component<T>
where
    T: 'static + Send + Sync,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
    }
}

mod alpha_tilemap_renderer;
mod audio_source;
mod camera;
mod diagnostic;
mod glyph_renderer;
mod nine_patch_renderer;
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
pub use size::*;
pub use sprite_renderer::*;
pub use tilemap_renderer::*;
pub use transform::*;
pub use ui_element::*;
pub use ui_scaler::*;

pub struct ComponentModule;

impl ModuleType for ComponentModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        alpha_tilemap_renderer::ComponentAlphaTilemapRenderer::register(&mut sub_module);
        audio_source::ComponentAudioSource::register(&mut sub_module);
        camera::ComponentCamera::register(&mut sub_module);
        diagnostic::ComponentDiagnostic::register(&mut sub_module);
        glyph_renderer::ComponentGlyphRenderer::register(&mut sub_module);
        glyph_renderer::GlyphLayoutConfig::register(&mut sub_module);
        glyph_renderer::HorizontalAlign::register(&mut sub_module);
        glyph_renderer::VerticalAlign::register(&mut sub_module);
        glyph_renderer::WrapStyle::register(&mut sub_module);
        nine_patch_renderer::ComponentNinePatchRenderer::register(&mut sub_module);
        size::ComponentSize::register(&mut sub_module);
        sprite_renderer::ComponentSpriteRenderer::register(&mut sub_module);
        tilemap_renderer::ComponentTilemapRenderer::register(&mut sub_module);
        transform::ComponentTransform::register(&mut sub_module);
        ui_element::ComponentUIElement::register(&mut sub_module);
        ui_scaler::ComponentUIScaler::register(&mut sub_module);
        ui_scaler::UIScaleMode::register(&mut sub_module);

        module.set_sub_module("component", sub_module);
    }
}

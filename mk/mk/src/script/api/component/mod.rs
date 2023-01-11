use crate::{engine::use_context, script::api::LuaApiTable};
use mlua::prelude::*;
use specs::prelude::*;
use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug)]
pub struct Component<T>
where
    T: specs::Component,
{
    entity: Entity,
    _marker: PhantomData<T>,
}

impl<T> Component<T>
where
    T: specs::Component,
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
        use_context()
            .world()
            .read_component::<T>()
            .get(self.entity)
            .map(|component| f(component))
    }

    pub fn with_mut<R>(self, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        use_context()
            .world()
            .write_component::<T>()
            .get_mut(self.entity)
            .map(|component| f(component))
    }
}

impl<T> Clone for Component<T>
where
    T: specs::Component,
{
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Copy for Component<T> where T: specs::Component {}

impl<T> PartialEq for Component<T>
where
    T: specs::Component,
{
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl<T> Eq for Component<T> where T: specs::Component {}

impl<T> Hash for Component<T>
where
    T: specs::Component,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
    }
}

// mod alpha_tilemap_renderer;
mod audio_source;
mod camera;
mod diagnostic;
mod glyph_renderer;
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
pub use size::*;
pub use sprite_renderer::*;
// pub use tilemap_renderer::*;
pub use transform::*;
pub use ui_element::*;
pub use ui_mask::*;
pub use ui_scaler::*;

pub struct ComponentModule;

impl LuaApiTable for ComponentModule {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        Ok(table)
    }
}

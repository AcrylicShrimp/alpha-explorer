use mlua::prelude::*;

trait IntoShared {
    type Shared;
    fn into_shared(self) -> Self::Shared;
}

macro_rules! define_shared_type {
    ($name:ident, $ty:ty) => {
        #[derive(Clone)]
        pub struct $name(std::sync::Arc<$ty>);

        impl $name {
            pub fn new(inner: $ty) -> Self {
                Self(std::sync::Arc::new(inner))
            }

            pub fn wrap(inner: std::sync::Arc<$ty>) -> Self {
                Self(inner)
            }

            pub fn inner(&self) -> std::sync::Arc<$ty> {
                self.0.clone()
            }

            pub fn into_inner(self) -> std::sync::Arc<$ty> {
                self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = std::sync::Arc<$ty>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl crate::script::api::IntoShared for $ty {
            type Shared = $name;

            fn into_shared(self) -> Self::Shared {
                $name::new(self)
            }
        }

        impl crate::script::api::IntoShared for std::sync::Arc<$ty> {
            type Shared = $name;

            fn into_shared(self) -> Self::Shared {
                $name::wrap(self)
            }
        }

        #[allow(dead_code)]
        type Inner = $ty;
    };
}

pub trait LuaApiTable {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>>;
}

pub mod asset;
pub mod audio;
pub mod component;
pub mod entity;
pub mod event;
pub mod glyph;
pub mod render;
pub mod screen;
pub mod structure;
pub mod time;
pub mod ui;

pub struct Module;

impl LuaApiTable for Module {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("asset", asset::AssetModule::create_api_table(lua)?)?;
        table.set("audio", audio::AudioModule::create_api_table(lua)?)?;
        table.set(
            "component",
            component::ComponentModule::create_api_table(lua)?,
        )?;
        table.set("entity", entity::EntityModule::create_api_table(lua)?)?;
        table.set("event", event::EventModule::create_api_table(lua)?)?;
        table.set("glyph", glyph::GlyphModule::create_api_table(lua)?)?;
        table.set("render", render::RenderModule::create_api_table(lua)?)?;
        table.set("screen", screen::ScreenModule::create_api_table(lua)?)?;
        table.set(
            "structure",
            structure::StructureModule::create_api_table(lua)?,
        )?;
        table.set("time", time::TimeModule::create_api_table(lua)?)?;
        table.set("ui", ui::UIModule::create_api_table(lua)?)?;

        Ok(table)
    }
}

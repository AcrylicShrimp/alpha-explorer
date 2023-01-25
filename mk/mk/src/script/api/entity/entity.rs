use crate::{
    component::*,
    engine::use_context,
    event::{EntityEventHandler, LuaEvent},
    script::{
        api::{component::*, LuaApiTable},
        FFIFunction,
    },
};
use mlua::prelude::*;
use smartstring::SmartString;
use specs::WorldExt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub specs::Entity);

impl Entity {
    pub fn new(entity: specs::Entity) -> Self {
        Self(entity)
    }

    pub fn with_ref<T, R>(self, f: impl FnOnce(&T) -> R) -> Option<R>
    where
        T: specs::Component,
    {
        use_context()
            .world()
            .read_storage::<T>()
            .get(self.0)
            .map(|component| f(component))
    }

    pub fn with_mut<T, R>(self, f: impl FnOnce(&mut T) -> R) -> Option<R>
    where
        T: specs::Component,
    {
        use_context()
            .world()
            .write_storage::<T>()
            .get_mut(self.0)
            .map(|component| f(component))
    }
}

impl LuaApiTable for Entity {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "find_by_name",
            lua.create_function(|_lua, name: LuaString| {
                let transform_mgr = use_context().transform_mgr();
                let allocator = transform_mgr.allocator();
                Ok(transform_mgr
                    .name_manager()
                    .transforms_by_name(name.to_str()?)
                    .first()
                    .copied()
                    .map(|index| Self::new(allocator.entity(index))))
            })?,
        )?;
        table.set(
            "find_all_by_name",
            lua.create_function(|_lua, name: LuaString| {
                let transform_mgr = use_context().transform_mgr();
                let allocator = transform_mgr.allocator();
                Ok(transform_mgr
                    .name_manager()
                    .transforms_by_name(name.to_str()?)
                    .iter()
                    .copied()
                    .map(|index| Self::new(allocator.entity(index)))
                    .collect::<Vec<_>>())
            })?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for Entity {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_lua, this| {
            Ok(this
                .with_ref(|component: &Transform| component.index())
                .map(|index| {
                    use_context()
                        .transform_mgr()
                        .name_manager()
                        .name(index)
                        .map(|name| name.to_string())
                }))
        });
        fields.add_field_method_set("name", |_lua, this, name: Option<String>| {
            this.with_ref(|component: &Transform| component.index())
                .map(|index| {
                    use_context()
                        .transform_mgr_mut()
                        .name_manager_mut()
                        .set_name(index, name.map(SmartString::from))
                });
            Ok(())
        });

        // fields.add_field_method_get("alpha_tilemap_renderer", |_lua, this| {
        //     Ok(ComponentAlphaTilemapRenderer::new(this.0))
        // });
        fields.add_field_method_get("audio_source", |_lua, this| {
            Ok(ComponentAudioSource::new(this.0))
        });
        fields.add_field_method_get("camera", |_lua, this| Ok(ComponentCamera::new(this.0)));
        fields.add_field_method_get("diagnostic", |_lua, this| {
            Ok(ComponentDiagnostic::new(this.0))
        });
        fields.add_field_method_get("glyph_renderer", |_lua, this| {
            Ok(ComponentGlyphRenderer::new(this.0))
        });
        fields.add_field_method_get("size", |_lua, this| Ok(ComponentSize::new(this.0)));
        fields.add_field_method_get("sprite_renderer", |_lua, this| {
            Ok(ComponentSpriteRenderer::new(this.0))
        });
        // fields.add_field_method_get("tilemap_renderer", |_lua, this| {
        //     Ok(ComponentTilemapRenderer::new(this.0))
        // });
        fields.add_field_method_get("transform", |_lua, this| {
            Ok(this.with_ref(|component: &Transform| ComponentTransform::new(component.index())))
        });
        fields.add_field_method_get("ui_element", |_lua, this| {
            Ok(ComponentUIElement::new(this.0))
        });
        fields.add_field_method_get("ui_mask", |_lua, this| Ok(ComponentUIMask::new(this.0)));
        fields.add_field_method_get("ui_scaler", |_lua, this| Ok(ComponentUIScaler::new(this.0)));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method(
            "listen",
            |lua, this, (event_name, handler): (LuaString, LuaValue)| {
                Ok(use_context().entity_event_mgr().add_handler(
                    *this,
                    event_name.to_str()?,
                    match handler {
                        LuaNil => todo!(),
                        LuaValue::Function(handler) => {
                            EntityEventHandler::lua(FFIFunction::new(lua, handler)?)
                        }
                        LuaValue::UserData(handler) => {
                            handler.borrow::<EntityEventHandler>()?.clone()
                        }
                        _ => return Err(LuaError::external("invalid event handler type")),
                    },
                ))
            },
        );
        methods.add_method(
            "unlisten",
            |_lua, this, (event_name, handler): (LuaString, EntityEventHandler)| {
                use_context().entity_event_mgr().remove_handler(
                    *this,
                    event_name.to_str()?,
                    handler,
                );
                Ok(())
            },
        );
        methods.add_method("emit", |lua, this, event: LuaMultiValue| {
            use_context().entity_event_mgr().emit(
                *this,
                &LuaEvent::from_lua_multi(event, lua)?,
                lua,
            );
            Ok(())
        });
    }
}

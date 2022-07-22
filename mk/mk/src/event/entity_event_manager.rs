use crate::event::events::PerEntity;
use crate::util::BoxId;
use legion::Entity;
use mlua::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct EntityEventManager {
    listeners_per_entity: HashMap<Entity, Vec<(String, BoxId<LuaRegistryKey>)>>,
}

impl EntityEventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entity_listener<'lua>(
        &mut self,
        lua: &'lua Lua,
        function: LuaFunction<'lua>,
        event: String,
        entity: Entity,
    ) -> LuaResult<usize> {
        let handler = BoxId::new(lua.create_registry_value(function)?);
        let hash = handler.hash();
        self.listeners_per_entity
            .entry(entity)
            .or_default()
            .push((event, handler));
        Ok(hash)
    }

    pub fn remove_entity_listener(&mut self, entity: Entity, hash: usize) {
        if let Some(listeners) = self.listeners_per_entity.get_mut(&entity) {
            if let Some(index) = listeners
                .iter()
                .position(|(.., listener)| listener.hash() == hash)
            {
                listeners.swap_remove(index);
            }
        }
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.listeners_per_entity.remove(&entity);
    }

    pub fn emit<'lua>(&self, lua: &'lua Lua, event: &'lua PerEntity<'lua>) -> LuaResult<()> {
        if let Some(listeners) = self.listeners_per_entity.get(&event.entity) {
            for (event_ty, listener) in listeners {
                if event_ty != &event.event {
                    continue;
                }

                lua.registry_value::<LuaFunction>(&listener)?
                    .call::<_, ()>(event.param.clone())?;
            }
        }

        Ok(())
    }
}

use super::{EntityEventHandler, Event};
use crate::script::entity::Entity;
use mlua::prelude::*;
use smartstring::alias::String;
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

#[derive(Default)]
pub struct EntityEventManager {
    per_entity: RefCell<HashMap<EntityEventKey, Vec<EntityEventHandler>>>,
    added_handlers: RefCell<Vec<(EntityEventKey, EntityEventHandler)>>,
    removed_handlers: RefCell<Vec<(EntityEventKey, EntityEventHandler)>>,
}

impl EntityEventManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_handler(
        &self,
        entity: Entity,
        event_name: impl Into<String>,
        handler: EntityEventHandler,
    ) -> EntityEventHandler {
        let key = EntityEventKey::new(entity, event_name);

        self.removed_handlers
            .borrow_mut()
            .retain(|(k, h)| k != &key || h != &handler);

        if let Ok(mut per_entity) = self.per_entity.try_borrow_mut() {
            per_entity.entry(key).or_default().push(handler.clone());
        } else {
            self.added_handlers
                .borrow_mut()
                .push((key, handler.clone()));
        }

        handler
    }

    pub fn remove_handler(
        &self,
        entity: Entity,
        event_name: impl Into<String>,
        handler: EntityEventHandler,
    ) {
        let key = EntityEventKey::new(entity, event_name);

        self.added_handlers
            .borrow_mut()
            .retain(|(k, h)| k != &key || h != &handler);

        if let Ok(mut per_entity) = self.per_entity.try_borrow_mut() {
            if let Some(handlers) = per_entity.get_mut(&key) {
                handlers.retain(|h| h != &handler);
            }
        } else {
            self.removed_handlers.borrow_mut().push((key, handler));
        }
    }

    pub fn emit(&self, entity: Entity, event: &dyn Event, lua: &Lua) {
        let mut per_entity = self.per_entity.borrow_mut();

        {
            let mut added_handlers = self.added_handlers.borrow_mut();
            for (key, handler) in added_handlers.drain(..) {
                match per_entity.entry(key) {
                    Entry::Occupied(mut entry) => entry.get_mut().push(handler),
                    Entry::Vacant(entry) => {
                        entry.insert(vec![handler]);
                    }
                }
            }
        }

        {
            let mut removed_handlers = self.removed_handlers.borrow_mut();
            for (key, handler) in removed_handlers.drain(..) {
                if let Some(handlers) = per_entity.get_mut(&key) {
                    handlers.retain(|h| h != &handler);
                }
            }
        }

        let key = EntityEventKey::new(entity, event.name());
        if let Some(handlers) = per_entity.get_mut(&key) {
            for handler in handlers {
                match handler.handle(entity, event, lua) {
                    Ok(..) => {}
                    Err(err) => {
                        // TODO: Report this error.
                        println!("Error handling event: {}", err);
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct EntityEventKey(Entity, String);

impl EntityEventKey {
    pub fn new(entity: Entity, event_name: impl Into<String>) -> Self {
        return Self(entity, event_name.into());
    }
}

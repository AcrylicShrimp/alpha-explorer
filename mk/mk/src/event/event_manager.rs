use crate::event::EventDispatcher;
use parking_lot::Mutex;
use specs::prelude::*;
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

#[derive(Default)]
pub struct EventManager {
    dispatcher: EventDispatcher,
    entity_dispatcher: Mutex<HashMap<Entity, Arc<EventDispatcher>>>,
}

impl EventManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn dispatcher(&self) -> &EventDispatcher {
        &self.dispatcher
    }

    pub fn entity_dispatcher(&self, entity: Entity) -> Arc<EventDispatcher> {
        match self.entity_dispatcher.lock().entry(entity) {
            Entry::Occupied(dispatcher) => dispatcher.get().clone(),
            Entry::Vacant(entry) => entry.insert(EventDispatcher::new().into()).clone(),
        }
    }

    pub fn remove_entity_dispatcher(&self, entity: Entity) -> Option<Arc<EventDispatcher>> {
        self.entity_dispatcher.lock().remove(&entity)
    }
}

use super::{Event, EventHandler};
use mlua::prelude::*;
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
};

#[derive(Default)]
pub struct EventManager {
    per_event: RefCell<HashMap<String, Vec<EventHandler>>>,
    added_handlers: RefCell<Vec<(String, EventHandler)>>,
    removed_handlers: RefCell<Vec<(String, EventHandler)>>,
}

impl EventManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_handler(
        &self,
        event_name: impl Into<String>,
        handler: EventHandler,
    ) -> EventHandler {
        let event_name = event_name.into();

        self.removed_handlers
            .borrow_mut()
            .retain(|(k, h)| k != &event_name || h != &handler);

        if let Ok(mut per_entity) = self.per_event.try_borrow_mut() {
            per_entity
                .entry(event_name)
                .or_default()
                .push(handler.clone());
        } else {
            self.added_handlers
                .borrow_mut()
                .push((event_name, handler.clone()));
        }

        handler
    }

    pub fn remove_handler(&self, event_name: impl Into<String>, handler: EventHandler) {
        let event_name = event_name.into();

        self.added_handlers
            .borrow_mut()
            .retain(|(k, h)| k != &event_name || h != &handler);

        if let Ok(mut per_event) = self.per_event.try_borrow_mut() {
            if let Some(handlers) = per_event.get_mut(&event_name) {
                handlers.retain(|h| h != &handler);
            }
        } else {
            self.removed_handlers
                .borrow_mut()
                .push((event_name, handler));
        }
    }

    pub fn emit(&self, event: &dyn Event, lua: &Lua) {
        let mut per_event = self.per_event.borrow_mut();

        {
            let mut added_handlers = self.added_handlers.borrow_mut();
            for (event_name, handler) in added_handlers.drain(..) {
                match per_event.entry(event_name) {
                    Entry::Occupied(mut entry) => entry.get_mut().push(handler),
                    Entry::Vacant(entry) => {
                        entry.insert(vec![handler]);
                    }
                }
            }
        }

        {
            let mut removed_handlers = self.removed_handlers.borrow_mut();
            for (event_name, handler) in removed_handlers.drain(..) {
                if let Some(handlers) = per_event.get_mut(&event_name) {
                    handlers.retain(|h| h != &handler);
                }
            }
        }

        let event_name = event.name();
        if let Some(handlers) = per_event.get(event_name) {
            for handler in handlers {
                match handler.handle(event, lua) {
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

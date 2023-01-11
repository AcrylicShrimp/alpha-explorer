use crate::{
    engine::use_context,
    event::{AbstractTypedEventBus, TypedEventBus, TypedEventListener},
    util::BoxId,
};
use mlua::prelude::*;
use parking_lot::{Mutex, MutexGuard};
use std::{
    any::{Any, TypeId},
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

#[derive(Default)]
pub struct EventDispatcher {
    event_buses: Mutex<HashMap<TypeId, Arc<dyn AbstractTypedEventBus>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            event_buses: HashMap::new().into(),
        }
    }

    pub fn listeners(&self) -> MutexGuard<HashMap<TypeId, Arc<dyn AbstractTypedEventBus>>> {
        self.event_buses.lock()
    }

    pub fn add_listener<T>(&self, listener: TypedEventListener<T>) -> usize
    where
        T: 'static + Any + Clone + for<'lua> ToLua<'lua>,
    {
        match self.event_buses.lock().entry(TypeId::of::<T>()) {
            Entry::Occupied(event_bus) => event_bus
                .get()
                .downcast_ref::<TypedEventBus<T>>()
                .unwrap()
                .add_listener(listener),
            Entry::Vacant(entry) => {
                let event_bus = TypedEventBus::<T>::new();
                let hash = event_bus.add_listener(listener);
                entry.insert(Arc::new(event_bus));
                hash
            }
        }
    }

    pub fn remove_listener<T>(&self, hash: usize) -> Option<BoxId<dyn Any>>
    where
        T: 'static + Any + Clone + for<'lua> ToLua<'lua>,
    {
        if let Some(event_bus) = self.event_buses.lock().get(&TypeId::of::<T>()) {
            event_bus.remove_listener(hash)
        } else {
            None
        }
    }

    pub fn emit<T>(&self, event: &T)
    where
        T: 'static + Any + Clone + for<'lua> ToLua<'lua>,
    {
        let script_mgr = use_context().script_mgr();

        if let Some(event_bus) = {
            let event_buses = self.event_buses.lock();
            event_buses.get(&TypeId::of::<T>()).cloned()
        } {
            event_bus
                .downcast_ref::<TypedEventBus<T>>()
                .unwrap()
                .handle(&script_mgr, event);
        }
    }

    pub fn emit_untyped<T>(&self, type_id: TypeId, event: impl AsRef<dyn Any>)
    where
        T: 'static + Any + Clone + for<'lua> ToLua<'lua>,
    {
        let event = event.as_ref();
        let script_mgr = use_context().script_mgr();

        if let Some(event_bus) = {
            let event_buses = self.event_buses.lock();
            event_buses.get(&type_id).cloned()
        } {
            event_bus.handle_untyped(script_mgr, event);
        }
    }
}

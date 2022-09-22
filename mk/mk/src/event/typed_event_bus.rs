use super::typed_event_listener::TypedEventListener;
use crate::{emit_diagnostic_error, script::ScriptManager, util::BoxId};
use downcast_rs::{impl_downcast, Downcast};
use mlua::prelude::*;
use parking_lot::Mutex;
use std::any::{type_name, Any};

pub trait AbstractTypedEventBus
where
    Self: Downcast,
{
    fn remove_listener(&self, hash: usize) -> Option<BoxId<dyn Any>>;
}

impl_downcast!(AbstractTypedEventBus);

pub struct TypedEventBus<T>
where
    T: 'static,
{
    listeners: Mutex<Vec<TypedEventListener<T>>>,
    added_listener_queue: Mutex<Option<Vec<TypedEventListener<T>>>>,
    removed_listener_queue: Mutex<Option<Vec<usize>>>,
}

impl<T> TypedEventBus<T>
where
    T: 'static,
{
    pub fn new() -> Self {
        Self {
            listeners: Vec::new().into(),
            added_listener_queue: None.into(),
            removed_listener_queue: None.into(),
        }
    }

    pub fn handle<'lua>(&self, script_mgr: &'lua ScriptManager, event: &T)
    where
        T: Clone + ToLua<'lua>,
    {
        // Prevent deadlock
        if let Some(mut listeners) = self.listeners.try_lock() {
            if let Some(added_listeners) = self.added_listener_queue.lock().take() {
                listeners.extend(added_listeners);
            }

            if let Some(removed_listeners) = self.removed_listener_queue.lock().take() {
                for hash in removed_listeners.into_iter() {
                    if let Some(index) = listeners
                        .iter()
                        .position(|listener| listener.hash() == hash)
                    {
                        listeners.swap_remove(index);
                    }
                }
            }

            for listener in listeners.iter_mut() {
                if let Err(err) = listener.listen(script_mgr, event) {
                    emit_diagnostic_error!(format!(
                        "an error occurred while handing {}: {:#}",
                        type_name::<T>(),
                        err
                    ));
                }
            }
        }
    }

    pub fn add_listener(&self, listener: TypedEventListener<T>) -> usize {
        let hash = listener.hash();

        match self.listeners.try_lock() {
            Some(mut listeners) => {
                listeners.push(listener);
            }
            None => {
                self.added_listener_queue
                    .lock()
                    .get_or_insert_with(|| Vec::new())
                    .push(listener);
            }
        }

        hash
    }
}

impl<T> AbstractTypedEventBus for TypedEventBus<T> {
    fn remove_listener(&self, hash: usize) -> Option<BoxId<dyn Any>> {
        match self.listeners.try_lock() {
            Some(mut listeners) => {
                if let Some(index) = listeners
                    .iter()
                    .position(|listener| listener.hash() == hash)
                {
                    listeners.swap_remove(index).upcast()
                } else {
                    None
                }
            }
            None => {
                self.removed_listener_queue
                    .lock()
                    .get_or_insert_with(|| Vec::new())
                    .push(hash);
                None
            }
        }
    }
}

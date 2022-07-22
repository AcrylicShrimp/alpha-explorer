use super::typed_event_listener::TypedEventListener;
use crate::emit_diagnostic_error;
use downcast_rs::{impl_downcast, Downcast};
use mlua::prelude::*;
use parking_lot::Mutex;
use std::any::type_name;

pub trait AbstractTypedEventBus
where
    Self: Downcast,
{
    fn remove_listener(&self, hash: usize);
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

    pub fn handle<'lua>(&self, lua: &'lua Lua, event: &T)
    where
        T: Clone + ToLua<'lua>,
    {
        // Prevent deadlock
        if let Some(mut listeners) = self.listeners.try_lock() {
            let mut index = 0;

            while index < listeners.len() {
                if let Err(err) = listeners[index].listen(lua, event) {
                    emit_diagnostic_error!(format!(
                        "an error occurred while handing {}: {}",
                        type_name::<T>(),
                        err
                    ));
                }

                index += 1;
            }

            match self.removed_listener_queue.lock().take() {
                Some(removed_listeners) => {
                    for hash in removed_listeners.into_iter() {
                        if let Some(index) = listeners
                            .iter()
                            .position(|listener| listener.hash() == hash)
                        {
                            listeners.swap_remove(index);
                        }
                    }
                }
                None => {}
            }

            if let Some(added_listeners) = self.added_listener_queue.lock().take() {
                listeners.extend(added_listeners);
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

impl<T> AbstractTypedEventBus for TypedEventBus<T>
where
    T: 'static,
{
    fn remove_listener(&self, hash: usize) {
        match self.listeners.try_lock() {
            Some(mut listeners) => {
                if let Some(index) = listeners
                    .iter()
                    .position(|listener| listener.hash() == hash)
                {
                    listeners.swap_remove(index);
                }
            }
            None => {
                self.removed_listener_queue
                    .lock()
                    .get_or_insert_with(|| Vec::new())
                    .push(hash);
            }
        }
    }
}

use crate::event::EventDispatcher;

#[derive(Default)]
pub struct EventManager {
    dispatcher: EventDispatcher,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            dispatcher: EventDispatcher::new(),
        }
    }

    pub fn dispatcher(&self) -> &EventDispatcher {
        &self.dispatcher
    }
}

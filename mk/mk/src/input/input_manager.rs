use crate::input::InputType;
use glutin::event::KeyboardInput;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub type Input = Arc<Mutex<dyn InputType>>;

pub struct InputManager {
    inputs: HashMap<String, Input>,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            inputs: HashMap::new(),
        }
    }

    pub fn input<S: AsRef<String>>(&self, name: S) -> Option<Input> {
        self.inputs.get(name.as_ref()).cloned()
    }

    pub fn register<T: InputType + 'static>(&mut self, name: String, input: T) -> Input {
        let input = Arc::new(Mutex::new(input));
        self.inputs.insert(name, input.clone());
        input
    }

    pub fn unregister<S: AsRef<String>>(&mut self, name: S) {
        self.inputs.remove(name.as_ref());
    }

    pub fn handle_event(&mut self, input: &KeyboardInput) {
        for input_type in self.inputs.values() {
            input_type.lock().handle_event(input);
        }
    }
}

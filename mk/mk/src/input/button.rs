use crate::input::InputType;
use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(Debug)]
pub struct Button {
    keycode: VirtualKeyCode,
    pressed: bool,
}

impl Button {
    pub fn new(keycode: VirtualKeyCode) -> Button {
        Button {
            keycode,
            pressed: false,
        }
    }
}

impl InputType for Button {
    fn value(&self) -> f32 {
        if self.pressed {
            1f32
        } else {
            -1f32
        }
    }

    fn reset(&mut self) {
        self.pressed = false;
    }

    fn handle_event(&mut self, input: &KeyboardInput) {
        match input.virtual_keycode {
            Some(keycode) => {
                if self.keycode == keycode {
                    self.pressed = input.state == ElementState::Pressed;
                }
            }
            None => {}
        }
    }
}

use crate::input::Driver;
use codegen::{Event, LuaEnum};
use winit::event::KeyboardInput as Test;

#[derive(Default)]
pub struct Keyboard {
    inputs: Vec<KeyboardInput>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_input(&mut self, input: KeyboardInput) {
        self.inputs.push(input);
    }
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardInput {
    pub scan_code: u32,
    pub key_code: Option<KeyCode>,
    pub is_pressed: bool,
}

#[derive(LuaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Escape,
    Space,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    AlphaA,
    AlphaB,
    AlphaC,
    AlphaD,
    AlphaE,
    AlphaF,
    AlphaG,
    AlphaH,
    AlphaI,
    AlphaJ,
    AlphaK,
    AlphaL,
    AlphaM,
    AlphaN,
    AlphaO,
    AlphaP,
    AlphaQ,
    AlphaR,
    AlphaS,
    AlphaT,
    AlphaU,
    AlphaV,
    AlphaW,
    AlphaX,
    AlphaY,
    AlphaZ,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLock,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    NumpadAdd,
    NumpadEnter,
    NumpadDecimal,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Grave,
    Minus,
    Equals,
    Backspace,
    Tab,
    LeftBracket,
    RightBracket,
    Backslash,
    CapsLock,
    Semicolon,
    Apostrophe,
    Enter,
    Comma,
    Period,
    Slash,
}

// impl Driver for Keyboard {
//     type Input = ();

//     fn update(&mut self, _context: &crate::EngineContext) {
//         for input in &self.inputs {}

//         self.inputs.clear();
//     }

//     fn create_trigger(&mut self, input: Self::Input) -> Box<dyn crate::input::Trigger> {
//         todo!()
//     }
// }

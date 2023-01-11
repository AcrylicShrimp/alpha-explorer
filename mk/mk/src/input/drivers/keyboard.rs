use crate::input::Driver;
use winit::event::KeyboardInput;

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

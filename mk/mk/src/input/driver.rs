use crate::{input::Trigger, EngineContext};
use mlua::prelude::*;

pub trait Driver {
    type Input: Clone + for<'lua> ToLua<'lua>;

    fn init(&mut self, _context: &EngineContext) {}
    fn update(&mut self, _context: &EngineContext) {}
    fn create_trigger(&mut self, input: Self::Input) -> Box<dyn Trigger>;
}

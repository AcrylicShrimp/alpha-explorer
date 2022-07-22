use crate::EngineContextWithoutSystemManager;

pub trait System {
    fn run(&mut self, context: &EngineContextWithoutSystemManager);
}

impl<F> System for F
where
    F: FnMut(&EngineContextWithoutSystemManager),
{
    fn run(&mut self, context: &EngineContextWithoutSystemManager) {
        self(context);
    }
}

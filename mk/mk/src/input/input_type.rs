use glutin::event::KeyboardInput;

pub trait InputType
where
    Self: Send,
{
    fn value(&self) -> f32;
    fn reset(&mut self);
    fn handle_event(&mut self, input: &KeyboardInput);
}

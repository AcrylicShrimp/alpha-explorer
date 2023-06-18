use super::Device;
use codegen::Event;
use std::sync::Arc;

pub trait InputChannel {
    fn name(&self) -> &str;
    fn device(&self) -> Arc<dyn Device>;
    fn state(&self) -> InputChannelValue;
}

#[derive(Debug, Clone, Copy)]
pub enum InputChannelValue {
    Button(bool),
    Axis1D(f32),
    Axis2D(f32, f32),
}

// #[derive(Event, Clone)]
// pub struct InputChannelStateChangedEvent {
//     pub channel: Arc<dyn InputChannel>,
//     pub old_state: InputChannelValue,
//     pub new_state: InputChannelValue,
// }

pub trait OutputChannel {
    fn name(&self) -> &str;
    fn device(&self) -> Arc<dyn Device>;
    fn state(&self) -> OutputChannelValue;
    fn set_state(&self, state: OutputChannelValue);
}

#[derive(Debug, Clone, Copy)]
pub enum OutputChannelValue {
    Bool(bool),
    Int(i64),
    Float(f64),
}

use super::{InputChannel, OutputChannel};
use std::sync::Arc;

pub trait Device {
    fn name(&self) -> &str;
    fn input_channels(&self) -> &[Arc<dyn InputChannel>];
    fn output_channels(&self) -> &[Arc<dyn OutputChannel>];
}

use super::Device;
use std::sync::Arc;

pub trait Driver {
    fn name(&self) -> &str;
    fn devices(&self) -> &[Arc<dyn Device>];
}

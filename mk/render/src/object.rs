use crate::NativeHandle;

pub trait Object {
    fn handle(&self) -> NativeHandle;
}

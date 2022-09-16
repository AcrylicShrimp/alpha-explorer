use specs::{prelude::*, Component};

#[derive(Component, Default, Clone, Copy)]
#[storage(NullStorage)]
pub struct Diagnostic;

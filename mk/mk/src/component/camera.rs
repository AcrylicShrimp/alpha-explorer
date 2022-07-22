use crate::render::Layer;
use codegen::{Animation, LuaComponent};

#[derive(Animation, LuaComponent, Debug)]
pub struct Camera {
    pub layer: Layer,
    pub order: isize,
}

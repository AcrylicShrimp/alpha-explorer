use crate::structure::Vec2;
use codegen::LuaStruct;

#[derive(LuaStruct, Default, Debug, Clone, PartialEq)]
pub struct UIAnchor {
    pub min: Vec2,
    pub max: Vec2,
}

impl UIAnchor {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
}

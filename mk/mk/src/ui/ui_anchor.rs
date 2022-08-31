use crate::structure::Vec2;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct UIAnchor {
    pub min: Vec2,
    pub max: Vec2,
}

impl UIAnchor {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn full() -> Self {
        Self {
            min: Vec2::zero(),
            max: Vec2::one(),
        }
    }
}

impl Display for UIAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UIAnchor(min={}, max={})", self.min, self.max)
    }
}

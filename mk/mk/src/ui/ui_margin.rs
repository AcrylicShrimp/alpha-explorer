use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct UIMargin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl UIMargin {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn zero() -> Self {
        Self {
            left: 0f32,
            right: 0f32,
            top: 0f32,
            bottom: 0f32,
        }
    }
}

impl Display for UIMargin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UIMargin(left={}, right={}, top={}, bottom={})",
            self.left, self.right, self.top, self.bottom
        )
    }
}

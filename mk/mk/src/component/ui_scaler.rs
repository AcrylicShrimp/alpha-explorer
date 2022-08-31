use crate::structure::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIScaleMode {
    Constant,
    Stretch,
    Fit,
    Fill,
    MatchWidth,
    MatchHeight,
}

pub struct UIScaler {
    pub mode: UIScaleMode,
    pub reference_size: Size,
}

impl UIScaler {
    pub fn new(mode: UIScaleMode, reference_size: Size) -> Self {
        Self {
            mode,
            reference_size,
        }
    }
}

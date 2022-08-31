use crate::render::Color;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct AlphaTile {
    pub fore_color: Color,
    pub back_color: Color,
    pub character: char,
}

impl AlphaTile {
    pub fn new(fore_color: Color, back_color: Color, character: char) -> Self {
        Self {
            fore_color,
            back_color,
            character,
        }
    }
}

impl Display for AlphaTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AlphaTile(fore_color={}, back_color={}, character={})",
            self.fore_color, self.back_color, self.character
        )
    }
}

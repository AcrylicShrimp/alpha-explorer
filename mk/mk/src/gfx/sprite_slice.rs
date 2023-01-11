use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteSlice {
    pub x_min: u16,
    pub x_max: u16,
    pub y_min: u16,
    pub y_max: u16,
}

impl SpriteSlice {
    pub fn new(x_min: u16, x_max: u16, y_min: u16, y_max: u16) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn min(self) -> (u16, u16) {
        (self.x_min, self.y_min)
    }

    pub fn max(self) -> (u16, u16) {
        (self.x_max, self.y_max)
    }
}

impl Display for SpriteSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SpriteSlice({},{}:{},{})",
            self.x_min, self.y_min, self.x_max, self.y_max,
        )
    }
}

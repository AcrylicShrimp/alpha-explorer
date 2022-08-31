use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Layer(u64);

impl Layer {
    pub fn new(layer: u64) -> Self {
        Self(layer)
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub fn has_overlap(lhs: Self, rhs: Self) -> bool {
        lhs.0 & rhs.0 != 0
    }

    pub fn none() -> Self {
        Self(0)
    }

    pub fn all() -> Self {
        Self(!0)
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Layer(flags={:064b})", self.0)
    }
}

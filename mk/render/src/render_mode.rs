#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderMode {
    Trangles,
}

impl RenderMode {
    pub fn count(self) -> u32 {
        match self {
            RenderMode::Trangles => 3,
        }
    }
}

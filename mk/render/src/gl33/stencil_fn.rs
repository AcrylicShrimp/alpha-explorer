#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum StencilFn {
    Never,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Eq,
    NotEq,
    Always,
}

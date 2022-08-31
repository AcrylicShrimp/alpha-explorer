#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenLiteralKind {
    Nil,
    Bool,
    Integer,
    Float,
    Str,
}

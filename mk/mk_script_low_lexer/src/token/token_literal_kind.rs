use crate::TokenStrLiteral;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum TokenLiteralKind {
    Integer,
    Float,
    DoubleQuotedStr(TokenStrLiteral),
}

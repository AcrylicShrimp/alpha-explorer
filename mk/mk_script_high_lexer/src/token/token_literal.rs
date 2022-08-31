use crate::{Symbol, TokenLiteralKind};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TokenLiteral {
    kind: TokenLiteralKind,
    str: Symbol,
}

impl TokenLiteral {
    pub fn new(kind: TokenLiteralKind, str: Symbol) -> Self {
        Self { kind, str }
    }

    pub fn kind(&self) -> TokenLiteralKind {
        self.kind
    }

    pub fn str(&self) -> Symbol {
        self.str
    }
}

use high_lexer::{Symbol, TokenLiteral};
use span::{Source, Span};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolWithSpan {
    pub symbol: Symbol,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeReference {
    pub module: Option<SymbolWithSpan>,
    pub name: SymbolWithSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub source: Arc<Source>,
    pub top_levels: Vec<TopLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopLevel {
    pub kind: TopLevelKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TopLevelKind {
    Use(TopLevelUse),
    Type(TopLevelType),
    Function(TopLevelFunction),
    Method(TopLevelMethod),
    Stmt(TopLevelStmt),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopLevelUse {
    pub path: TokenLiteral,
    pub name: Option<SymbolWithSpan>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopLevelType {
    pub name: SymbolWithSpan,
    pub from: SymbolWithSpan,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopLevelFunction {
    pub name: SymbolWithSpan,
    pub params: Vec<SymbolWithSpan>,
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopLevelMethod {
    pub name: SymbolWithSpan,
    pub ty: Option<TypeReference>,
    pub params: Vec<SymbolWithSpan>,
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopLevelStmt {
    pub kind: TopLevelStmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TopLevelStmtKind {
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtKind {
    Block(StmtBlock),
    If(StmtIf),
    For(StmtFor),
    Match(StmtMatch),
    Function(Function),
    Break,
    Continue,
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtBlock {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtIf {
    pub condition: Expr,
    pub then_block: StmtBlock,
    pub else_kind: Option<StmtElseKind>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtElseKind {
    ElseIf(Box<StmtIf>),
    Else(StmtBlock),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtFor {
    pub kind: StmtForKind,
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtMatch {
    pub kind: StmtForKind,
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtForKind {
    Loop,
    While(StmtForKindWhile),
    ForEach(StmtForKindForEach),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtForKindWhile {
    pub condition: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtForKindForEach {
    pub params: Vec<SymbolWithSpan>,
    pub expr: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    pub name: SymbolWithSpan,
    pub params: Vec<SymbolWithSpan>,
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub span: Span,
}

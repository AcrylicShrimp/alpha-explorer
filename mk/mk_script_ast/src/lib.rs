use high_lexer::{Symbol, TokenLiteral};
use span::{Source, Span};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolWithSpan {
    pub symbol: Symbol,
    pub span: Span,
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
    pub path: Literal,
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
    pub for_ty: TypeReference,
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
    Block(StmtBlock),
    Let(StmtLet),
    If(StmtIf),
    Loop(StmtLoop),
    While(StmtWhile),
    For(StmtFor),
    Match(StmtMatch),
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
    Let(StmtLet),
    If(StmtIf),
    Loop(StmtLoop),
    While(StmtWhile),
    For(StmtFor),
    Match(StmtMatch),
    Break,
    Continue,
    Return(Option<Expr>),
    Assign(StmtAssign),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtBlock {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtLet {
    pub name: SymbolWithSpan,
    pub expr: Option<Expr>,
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
pub struct StmtLoop {
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtWhile {
    pub condition: Expr,
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtFor {
    pub pattern: Pattern,
    pub expr: Expr,
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtMatch {
    pub expr: Expr,
    pub patterns: Vec<StmtMatchPattern>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtMatchPattern {
    pub kind: StmtMatchPatternKind,
    pub block: StmtBlock,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtMatchPatternKind {
    PatternList(Vec<Pattern>),
    PatternWithCondition(PatternWithCondition),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtAssign {
    pub lhs: Expr,
    pub rhs: Expr,
    pub kind: StmtAssignKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StmtAssignKind {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Shl,
    Shr,
    BitOr,
    BitAnd,
    BitXor,
    BitNot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeReference {
    pub module: Option<SymbolWithSpan>,
    pub name: SymbolWithSpan,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PatternWithCondition {
    pub kind: PatternKind,
    pub condition: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternKind {
    Wildcard,
    Literal(Literal),
    Ident(SymbolWithSpan),
    Enum(EnumReference),
    Dict(Vec<(SymbolWithSpan, Pattern)>),
    List(Vec<Pattern>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprKind {
    Rng(Box<Expr>, Box<Expr>),
    RngInclusive(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Shr(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    BitAnd(Box<Expr>, Box<Expr>),
    BitXor(Box<Expr>, Box<Expr>),
    BitNot(Box<Expr>),
    LogOr(Box<Expr>, Box<Expr>),
    LogAnd(Box<Expr>, Box<Expr>),
    LogNot(Box<Expr>),
    Cast(Box<Expr>, TypeReference),
    Call(Box<Expr>, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Member(Box<Expr>, SymbolWithSpan),
    Id(SymbolWithSpan),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumReference {
    pub ty: TypeReference,
    pub variant: SymbolWithSpan,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub literal: TokenLiteral,
    pub span: Span,
}

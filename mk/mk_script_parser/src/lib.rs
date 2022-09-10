mod cursor;
mod parser;

use crate::parser::*;
use ast::*;
use cursor::*;
use diagnostic::Diagnostic;
use high_lexer::*;
use span::{Source, Span};
use std::sync::Arc;

// TODO: Emit diagnostics to report syntax errors.
// TODO: Provide a way to report errors more easily.
// TODO: Fix the expectation of the parser to be correct.

enum TopLevelFunctionOrMethod {
    Function(TopLevelFunction),
    Method(TopLevelMethod),
}

pub fn parse(
    source: Arc<Source>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Module, (String, Span)> {
    let mut top_levels = vec![];

    {
        let mut parser = Parser::new(Cursor::new(token_iter(&source, diagnostics)));

        while parser.exists() {
            top_levels.push(parse_top_level(&mut parser)?);
        }
    }

    Ok(Module { source, top_levels })
}

fn parse_top_level(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<TopLevel, (String, Span)> {
    if parser.expect_keyword(USE) {
        parser.expect_begin();
        let item = parse_use(parser)?;

        Ok(TopLevel {
            span: item.span.to(parser.span()),
            kind: TopLevelKind::Use(item),
        })
    } else if parser.expect_keyword(TYPE) {
        parser.expect_begin();
        let item = parse_type(parser)?;

        Ok(TopLevel {
            span: item.span.to(parser.span()),
            kind: TopLevelKind::Type(item),
        })
    } else if parser.expect_keyword(FN) {
        parser.expect_begin();
        match parse_function_or_method(parser)? {
            TopLevelFunctionOrMethod::Function(item) => Ok(TopLevel {
                span: item.span.to(parser.span()),
                kind: TopLevelKind::Function(item),
            }),
            TopLevelFunctionOrMethod::Method(item) => Ok(TopLevel {
                span: item.span.to(parser.span()),
                kind: TopLevelKind::Method(item),
            }),
        }
    } else {
        let item = parse_top_level_stmt(parser)?;

        Ok(TopLevel {
            span: item.span.to(parser.span()),
            kind: TopLevelKind::Stmt(item),
        })
    }
}

fn parse_use(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<TopLevelUse, (String, Span)> {
    let span = parser.span();
    let path = if let Some(item) = parser.expect_str_literal() {
        Literal {
            span: parser.span(),
            literal: item,
        }
    } else {
        return Err(parser.expect_else());
    };

    parser.expect_begin();
    let name = if parser.expect_keyword(AS) {
        parser.expect_begin();
        Some(parser.expect_id().ok_or_else(|| parser.expect_else())?)
    } else {
        None
    };

    parser.expect_begin();
    if !parser.expect_kind(TokenKind::Semicolon) {
        return Err(parser.expect_else());
    }

    Ok(TopLevelUse {
        span: span.to(parser.span()),
        path,
        name,
    })
}

fn parse_type(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<TopLevelType, (String, Span)> {
    let span = parser.span();
    let name = parser.ensure_id()?;

    parser.expect_begin();
    if !parser.expect_keyword(FROM) {
        return Err(parser.expect_else());
    }

    parser.expect_begin();
    let from = parser.ensure_id()?;

    parser.expect_begin();
    if !parser.expect_kind(TokenKind::Semicolon) {
        return Err(parser.expect_else());
    }

    Ok(TopLevelType {
        name,
        from,
        span: span.to(parser.span()),
    })
}

fn parse_function_or_method(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<TopLevelFunctionOrMethod, (String, Span)> {
    let span = parser.span();
    let name = parser.ensure_id()?;

    parser.expect_begin();
    if !parser.expect_kind(TokenKind::OpenParen) {
        return Err(parser.expect_else());
    }

    let mut params = vec![];

    while !parser.expect_kind(TokenKind::CloseParen) {
        if !parser.exists() {
            return Err(parser.expect_else());
        }

        parser.expect_begin();
        params.push(parser.ensure_id()?);

        parser.expect_begin();
        parser.expect_kind(TokenKind::Comma);
    }

    parser.expect_begin();
    let for_ty = if parser.expect_keyword(FOR) {
        parser.expect_begin();
        Some(parse_type_reference(parser)?)
    } else {
        None
    };

    parser.expect_begin();
    let block = parse_block(parser)?;

    Ok(match for_ty {
        Some(for_ty) => TopLevelFunctionOrMethod::Method(TopLevelMethod {
            name,
            for_ty,
            params,
            block,
            span: span.to(parser.span()),
        }),
        None => TopLevelFunctionOrMethod::Function(TopLevelFunction {
            name,
            params,
            block,
            span: span.to(parser.span()),
        }),
    })
}

fn parse_top_level_stmt(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<TopLevelStmt, (String, Span)> {
    let span = parser.span();

    if parser.expect_kind(TokenKind::OpenBrace) {
        parser.expect_begin();
        Ok(TopLevelStmt {
            kind: TopLevelStmtKind::Block(parse_block(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(LET) {
        parser.expect_begin();
        Ok(TopLevelStmt {
            kind: TopLevelStmtKind::Let(parse_let(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(IF) {
        parser.expect_begin();
        Ok(TopLevelStmt {
            kind: TopLevelStmtKind::If(parse_if(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(LOOP) {
        parser.expect_begin();
        Ok(TopLevelStmt {
            kind: TopLevelStmtKind::Loop(parse_loop(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(WHILE) {
        parser.expect_begin();
        Ok(TopLevelStmt {
            kind: TopLevelStmtKind::While(parse_while(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(FOR) {
        parser.expect_begin();
        Ok(TopLevelStmt {
            kind: TopLevelStmtKind::For(parse_for(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(MATCH) {
        parser.expect_begin();
        Ok(TopLevelStmt {
            kind: TopLevelStmtKind::Match(parse_match(parser)?),
            span: span.to(parser.span()),
        })
    } else {
        let expr = parse_expr(parser)?;

        parser.expect_begin();
        if !parser.expect_kind(TokenKind::Semicolon) {
            return Err(parser.expect_else());
        }

        Ok(TopLevelStmt {
            kind: TopLevelStmtKind::Expr(expr),
            span: span.to(parser.span()),
        })
    }
}

fn parse_stmt(parser: &mut Parser<impl Iterator<Item = Token>>) -> Result<Stmt, (String, Span)> {
    let span = parser.span();

    if parser.expect_kind(TokenKind::OpenBrace) {
        parser.expect_begin();
        Ok(Stmt {
            kind: StmtKind::Block(parse_block(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(LET) {
        parser.expect_begin();
        Ok(Stmt {
            kind: StmtKind::Block(parse_block(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(IF) {
        parser.expect_begin();
        Ok(Stmt {
            kind: StmtKind::If(parse_if(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(LOOP) {
        parser.expect_begin();
        Ok(Stmt {
            kind: StmtKind::Loop(parse_loop(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(WHILE) {
        parser.expect_begin();
        Ok(Stmt {
            kind: StmtKind::While(parse_while(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(FOR) {
        parser.expect_begin();
        Ok(Stmt {
            kind: StmtKind::For(parse_for(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(MATCH) {
        parser.expect_begin();
        Ok(Stmt {
            kind: StmtKind::Match(parse_match(parser)?),
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(BREAK) {
        parser.expect_begin();
        if !parser.expect_kind(TokenKind::Semicolon) {
            return Err(parser.expect_else());
        }

        Ok(Stmt {
            kind: StmtKind::Break,
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(CONTINUE) {
        parser.expect_begin();
        if !parser.expect_kind(TokenKind::Semicolon) {
            return Err(parser.expect_else());
        }

        Ok(Stmt {
            kind: StmtKind::Continue,
            span: span.to(parser.span()),
        })
    } else if parser.expect_keyword(RETURN) {
        parser.expect_begin();
        let expr = if parser.expect_kind(TokenKind::Semicolon) {
            None
        } else {
            let expr = parse_expr(parser)?;

            parser.expect_begin();
            if !parser.expect_kind(TokenKind::Semicolon) {
                return Err(parser.expect_else());
            }

            Some(expr)
        };

        Ok(Stmt {
            kind: StmtKind::Return(expr),
            span: span.to(parser.span()),
        })
    } else {
        let expr = parse_expr(parser)?;

        parser.expect_begin();
        if !parser.expect_kind(TokenKind::Semicolon) {
            return Err(parser.expect_else());
        }

        Ok(Stmt {
            kind: StmtKind::Expr(expr),
            span: span.to(parser.span()),
        })
    }
}

fn parse_block(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<StmtBlock, (String, Span)> {
    let span = parser.span();
    let mut stmts = vec![];

    while !parser.expect_kind(TokenKind::CloseBrace) {
        if !parser.exists() {
            return Err(parser.expect_else());
        }

        parser.expect_begin();
        stmts.push(parse_stmt(parser)?);
    }

    Ok(StmtBlock {
        stmts,
        span: span.to(parser.span()),
    })
}

fn parse_let(parser: &mut Parser<impl Iterator<Item = Token>>) -> Result<StmtLet, (String, Span)> {
    let span = parser.span();
    let name = parser.ensure_id()?;

    parser.expect_begin();
    let expr = if parser.expect_kind(TokenKind::Assign) {
        parser.expect_begin();
        Some(parse_expr(parser)?)
    } else {
        None
    };

    parser.expect_begin();
    if !parser.expect_kind(TokenKind::Semicolon) {
        return Err(parser.expect_else());
    }

    Ok(StmtLet {
        span: span.to(parser.span()),
        name,
        expr,
    })
}

fn parse_if(parser: &mut Parser<impl Iterator<Item = Token>>) -> Result<StmtIf, (String, Span)> {
    let span = parser.span();
    let condition = parse_expr(parser)?;

    parser.expect_begin();
    if !parser.expect_kind(TokenKind::OpenBrace) {
        return Err(parser.expect_else());
    }
    let then_block = parse_block(parser)?;

    parser.expect_begin();
    Ok(if parser.expect_keyword(ELSE) {
        parser.expect_begin();
        if parser.expect_keyword(IF) {
            let else_if = parse_if(parser)?;

            StmtIf {
                condition,
                then_block,
                else_kind: Some(StmtElseKind::ElseIf(Box::new(else_if))),
                span: span.to(parser.span()),
            }
        } else if parser.expect_kind(TokenKind::OpenBrace) {
            let else_block = parse_block(parser)?;

            StmtIf {
                condition,
                then_block,
                else_kind: Some(StmtElseKind::Else(else_block)),
                span: span.to(parser.span()),
            }
        } else {
            return Err(parser.expect_else());
        }
    } else {
        StmtIf {
            condition,
            then_block,
            else_kind: None,
            span: span.to(parser.span()),
        }
    })
}

fn parse_loop(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<StmtLoop, (String, Span)> {
    let span = parser.span();

    if !parser.expect_kind(TokenKind::OpenBrace) {
        return Err(parser.expect_else());
    }
    let block = parse_block(parser)?;

    Ok(StmtLoop {
        block,
        span: span.to(parser.span()),
    })
}

fn parse_while(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<StmtWhile, (String, Span)> {
    let span = parser.span();
    let condition = parse_expr(parser)?;

    parser.expect_begin();
    if !parser.expect_kind(TokenKind::OpenBrace) {
        return Err(parser.expect_else());
    }
    let block = parse_block(parser)?;

    Ok(StmtWhile {
        condition,
        block,
        span: span.to(parser.span()),
    })
}

fn parse_for(parser: &mut Parser<impl Iterator<Item = Token>>) -> Result<StmtFor, (String, Span)> {
    let span = parser.span();
    let pattern = parse_pattern(parser)?;

    parser.expect_begin();
    if !parser.expect_keyword(IN) {
        return Err(parser.expect_else());
    }

    parser.expect_begin();
    let expr = parse_expr(parser)?;

    parser.expect_begin();
    if !parser.expect_kind(TokenKind::OpenBrace) {
        return Err(parser.expect_else());
    }
    let block = parse_block(parser)?;

    Ok(StmtFor {
        pattern,
        expr,
        block,
        span: span.to(parser.span()),
    })
}

fn parse_match(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<StmtMatch, (String, Span)> {
    let span = parser.span();
    let expr = parse_expr(parser)?;

    parser.expect_begin();
    if !parser.expect_kind(TokenKind::OpenBrace) {
        return Err(parser.expect_else());
    }

    let mut patterns = Vec::new();

    while !parser.expect_kind(TokenKind::CloseBrace) {
        if !parser.exists() {
            return Err(parser.expect_else());
        }

        parser.expect_begin();
        patterns.push(parse_match_pattern(parser)?);
    }

    Ok(StmtMatch {
        expr,
        patterns,
        span: span.to(parser.span()),
    })
}

fn parse_match_pattern(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<StmtMatchPattern, (String, Span)> {
    let span = parser.span();
    let item = parse_pattern(parser)?;

    parser.expect_begin();
    if parser.expect_keyword(IF) {
        parser.expect_begin();
        let condition = parse_expr(parser)?;

        parser.expect_begin();
        if !parser.expect_kind(TokenKind::Arrow) {
            return Err(parser.expect_else());
        }

        parser.expect_begin();
        let block = parse_block(parser)?;

        Ok(StmtMatchPattern {
            kind: StmtMatchPatternKind::PatternWithCondition(PatternWithCondition {
                span: item.span.to(condition.span),
                kind: item.kind,
                condition,
            }),
            block,
            span: span.to(parser.span()),
        })
    } else {
        let mut items = vec![item];

        while !parser.expect_kind(TokenKind::Arrow) {
            if !parser.exists() {
                return Err(parser.expect_else());
            }

            parser.expect_begin();
            items.push(parse_pattern(parser)?);
        }

        parser.expect_begin();
        let block = parse_block(parser)?;

        Ok(StmtMatchPattern {
            kind: StmtMatchPatternKind::PatternList(items),
            block,
            span: span.to(parser.span()),
        })
    }
}

fn parse_type_reference(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<TypeReference, (String, Span)> {
    let span = parser.span();
    let mut module = Some(if let Some(item) = parser.expect_id() {
        item
    } else {
        return Err(parser.expect_else());
    });

    parser.expect_begin();
    let name = if parser.expect_kind(TokenKind::Member) {
        let name = module.take().unwrap();

        parser.expect_begin();
        module = Some(if let Some(item) = parser.expect_id() {
            item
        } else {
            return Err(parser.expect_else());
        });

        name
    } else {
        module.take().unwrap()
    };

    Ok(TypeReference {
        module,
        name,
        span: span.to(parser.span()),
    })
}

fn parse_pattern(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Pattern, (String, Span)> {
    let span = parser.span();
    let item = parse_pattern_without_list(parser)?;

    parser.expect_begin();
    if parser.expect_kind(TokenKind::Comma) {
        parser.expect_begin();
        let mut list = vec![item];

        while parser.exists() {
            parser.expect_begin();
            if parser.expect_kind(TokenKind::Arrow)
                || parser.expect_kind(TokenKind::BitOr)
                || parser.expect_kind(TokenKind::Arrow)
                || parser.expect_keyword(IF)
                || parser.expect_keyword(IN)
            {
                break;
            }

            list.push(parse_pattern_without_list(parser)?);

            parser.expect_begin();
            if !parser.expect_kind(TokenKind::Comma) {
                break;
            }
        }

        Ok(Pattern {
            kind: PatternKind::List(list),
            span: span.to(parser.span()),
        })
    } else {
        parser.expect_begin();
        Ok(item)
    }
}

fn parse_pattern_without_list(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Pattern, (String, Span)> {
    let span = parser.span();

    if parser.expect_keyword(WILDCARD) {
        parser.expect_begin();
        Ok(Pattern {
            kind: PatternKind::Wildcard,
            span: span.to(parser.span()),
        })
    } else if let Some(item) = parser.expect_literal() {
        parser.expect_begin();
        Ok(Pattern {
            kind: PatternKind::Literal(Literal {
                literal: item,
                span: span.to(parser.span()),
            }),
            span: span.to(parser.span()),
        })
    } else if let Some(item) = parser.expect_id() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::Member) {
            let enum_name = item;

            parser.expect_begin();
            let enum_variant = if let Some(item) = parser.expect_id() {
                item
            } else {
                return Err(parser.expect_else());
            };

            parser.expect_begin();
            if parser.expect_kind(TokenKind::Member) {
                let module = enum_name;
                let enum_name = enum_variant;

                parser.expect_begin();
                let enum_variant = if let Some(item) = parser.expect_id() {
                    item
                } else {
                    return Err(parser.expect_else());
                };

                Ok(Pattern {
                    kind: PatternKind::Enum(EnumReference {
                        span: module.span.to(enum_variant.span),
                        ty: TypeReference {
                            span: module.span.to(enum_name.span),
                            module: Some(module),
                            name: enum_name,
                        },
                        variant: enum_variant,
                    }),
                    span: span.to(parser.span()),
                })
            } else {
                Ok(Pattern {
                    kind: PatternKind::Enum(EnumReference {
                        span: enum_name.span.to(enum_variant.span),
                        ty: TypeReference {
                            span: enum_name.span,
                            module: None,
                            name: enum_name,
                        },
                        variant: enum_variant,
                    }),
                    span: span.to(parser.span()),
                })
            }
        } else {
            Ok(Pattern {
                kind: PatternKind::Ident(item),
                span: span.to(parser.span()),
            })
        }
    } else if parser.expect_kind(TokenKind::OpenBracket) {
        let mut dict = Vec::new();

        parser.expect_begin();
        loop {
            if !parser.exists() {
                return Err(parser.expect_else());
            }

            parser.expect_begin();
            let key = parser.ensure_id()?;

            parser.expect_begin();
            let value = parse_pattern(parser)?;

            dict.push((key, value));

            parser.expect_begin();
            if !parser.expect_kind(TokenKind::Comma) {
                break;
            }

            parser.expect_begin();
            if parser.cursor().is_kind(TokenKind::CloseBracket) {
                break;
            }
        }

        parser.expect_begin();
        if !parser.expect_kind(TokenKind::CloseBracket) {
            return Err(parser.expect_else());
        }

        Ok(Pattern {
            kind: PatternKind::Dict(dict),
            span: span.to(parser.span()),
        })
    } else {
        return Err(parser.expect_else());
    }
}

fn parse_expr(parser: &mut Parser<impl Iterator<Item = Token>>) -> Result<Expr, (String, Span)> {
    parse_expr_binary_log_or(parser)
}

fn parse_expr_binary_log_or(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_log_and(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::LogOr) {
            let rhs = parse_expr_binary_log_and(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::LogOr(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_log_and(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_cmp(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::LogAnd) {
            let rhs = parse_expr_binary_cmp(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::LogAnd(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_cmp(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_rng(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::Eq) {
            let rhs = parse_expr_binary_rng(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Eq(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Ne) {
            let rhs = parse_expr_binary_rng(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Ne(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Lt) {
            let rhs = parse_expr_binary_rng(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Lt(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Gt) {
            let rhs = parse_expr_binary_rng(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Gt(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Le) {
            let rhs = parse_expr_binary_rng(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Le(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Ge) {
            let rhs = parse_expr_binary_rng(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Ge(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_rng(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_shift(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::Rng) {
            let rhs = parse_expr_binary_shift(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Rng(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::RngInclusive) {
            let rhs = parse_expr_binary_shift(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::RngInclusive(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_shift(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_addsub(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::Shl) {
            let rhs = parse_expr_binary_addsub(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Shl(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Shr) {
            let rhs = parse_expr_binary_addsub(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Shr(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_addsub(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_muldivmod(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::Add) {
            let rhs = parse_expr_binary_muldivmod(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Add(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Sub) {
            let rhs = parse_expr_binary_muldivmod(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Sub(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_muldivmod(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_bit_or(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::Mul) {
            let rhs = parse_expr_binary_bit_or(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Mul(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Div) {
            let rhs = parse_expr_binary_bit_or(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Div(Box::new(item), Box::new(rhs)),
            }
        } else if parser.expect_kind(TokenKind::Mod) {
            let rhs = parse_expr_binary_bit_or(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::Mod(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_bit_or(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_bit_and(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::BitOr) {
            let rhs = parse_expr_binary_bit_and(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::BitOr(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_bit_and(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_binary_bit_xor(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::BitAnd) {
            let rhs = parse_expr_binary_bit_xor(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::BitAnd(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_binary_bit_xor(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_as(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::BitXor) {
            let rhs = parse_expr_as(parser)?;
            item = Expr {
                span: item.span.to(rhs.span),
                kind: ExprKind::BitXor(Box::new(item), Box::new(rhs)),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_as(parser: &mut Parser<impl Iterator<Item = Token>>) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_unary(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_keyword(AS) {
            let ty = parse_type_reference(parser)?;
            item = Expr {
                span: item.span.to(ty.span),
                kind: ExprKind::Cast(Box::new(item), ty),
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_unary(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    if parser.expect_kind(TokenKind::LogNot) {
        let span = parser.span();

        parser.expect_begin();
        parse_expr_unary(parser).map(|expr| Expr {
            span: span.to(expr.span),
            kind: ExprKind::LogNot(Box::new(expr)),
        })
    } else if parser.expect_kind(TokenKind::Add) {
        parser.expect_begin();
        parse_expr_unary(parser)
    } else if parser.expect_kind(TokenKind::Sub) {
        let span = parser.span();

        parser.expect_begin();
        parse_expr_unary(parser).map(|expr| Expr {
            span: span.to(expr.span),
            kind: ExprKind::Neg(Box::new(expr)),
        })
    } else if parser.expect_kind(TokenKind::BitNot) {
        let span = parser.span();

        parser.expect_begin();
        parse_expr_unary(parser).map(|expr| Expr {
            span: span.to(expr.span),
            kind: ExprKind::BitNot(Box::new(expr)),
        })
    } else {
        parse_expr_single_and_member(parser)
    }
}

fn parse_expr_single_and_member(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    let mut item = parse_expr_item(parser)?;

    while parser.exists() {
        parser.expect_begin();
        if parser.expect_kind(TokenKind::OpenParen) {
            let mut args = vec![];

            while !parser.expect_kind(TokenKind::CloseParen) {
                if !parser.exists() {
                    return Err(parser.expect_else());
                }

                args.push(parse_expr(parser)?);

                parser.expect_begin();
                parser.expect_kind(TokenKind::Comma);
            }

            item = Expr {
                span: item.span.to(parser.span()),
                kind: ExprKind::Call(Box::new(item), args),
            }
        } else if parser.expect_kind(TokenKind::OpenBracket) {
            let expr = parse_expr(parser)?;

            parser.expect_begin();
            if !parser.expect_kind(TokenKind::CloseBracket) {
                return Err(parser.expect_else());
            }

            item = Expr {
                span: item.span.to(parser.span()),
                kind: ExprKind::Index(Box::new(item), Box::new(expr)),
            }
        } else if parser.expect_kind(TokenKind::Dot) {
            parser.expect_begin();
            if let Some(id) = parser.expect_id() {
                item = Expr {
                    span: item.span.to(parser.span()),
                    kind: ExprKind::Member(Box::new(item), id),
                }
            } else {
                return Err(parser.expect_else());
            }
        } else {
            break;
        }
    }

    Ok(item)
}

fn parse_expr_item(
    parser: &mut Parser<impl Iterator<Item = Token>>,
) -> Result<Expr, (String, Span)> {
    if parser.expect_kind(TokenKind::OpenParen) {
        let expr = parse_expr(parser)?;

        parser.expect_begin();
        if !parser.expect_kind(TokenKind::CloseParen) {
            return Err(parser.expect_else());
        }

        Ok(expr)
    } else if parser.expect_kind(TokenKind::OpenBracket) {
        let span = parser.span();
        let expr = parse_expr(parser)?;

        parser.expect_begin();
        if !parser.expect_kind(TokenKind::CloseBracket) {
            return Err(parser.expect_else());
        }

        Ok(Expr {
            kind: ExprKind::Deref(Box::new(expr)),
            span: span.to(parser.span()),
        })
    } else if let Some(id) = parser.expect_id() {
        let span = parser.span();
        parser.expect_begin();
        if parser.expect_kind(TokenKind::ModuleMember) {
            parser.expect_begin();
            if let Some(sub_id) = parser.expect_id() {
                Ok(Expr {
                    kind: ExprKind::ModuleMember(
                        SymbolWithSpan { symbol: id, span },
                        SymbolWithSpan {
                            symbol: sub_id,
                            span: parser.span(),
                        },
                    ),
                    span: span.to(parser.span()),
                })
            } else {
                Err(parser.expect_else())
            }
        } else {
            Ok(Expr {
                kind: ExprKind::Id(id),
                span: parser.span(),
            })
        }
    } else if let Some(literal) = parser.expect_literal() {
        Ok(Expr {
            kind: ExprKind::Literal(Literal {
                literal,
                span: parser.span(),
            }),
            span: parser.span(),
        })
    } else {
        Err(parser.expect_else())
    }
}

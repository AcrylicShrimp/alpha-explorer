mod symbols;
mod token;

pub use symbols::*;
pub use token::*;

use diagnostic::{Diagnostic, Level, MultiSpan};
use low_lexer::{
    token_iter as low_token_iter, Token as LowToken, TokenKind as LowTokenKind,
    TokenLiteralKind as LowTokenLiteralKind,
};
use span::{Pos, Source, Span};
use std::iter::from_fn as iter_from_fn;

pub fn token_iter<'s>(
    source: &'s Source,
    diagnostics: &'s mut Vec<Diagnostic>,
) -> impl Iterator<Item = Token> + 's {
    let mut iter = unglued_token_iter(source, diagnostics);
    let mut current = iter.next();
    let mut next = iter.next();

    iter_from_fn(move || {
        let mut token = match current.take() {
            Some(token) => token,
            None => return None,
        };

        while let Some(next_token) = next.take() {
            if let Some(glued) = token.glue(&next_token) {
                next = iter.next();
                token = glued;
            } else {
                next = Some(next_token);
                break;
            }
        }

        current = next.take();
        next = iter.next();
        Some(token)
    })
}

fn unglued_token_iter<'s>(
    source: &'s Source,
    diagnostics: &'s mut Vec<Diagnostic>,
) -> impl Iterator<Item = Token> + 's {
    let mut low = source.span().low();
    let mut iter = low_token_iter(source.content());

    iter_from_fn(move || loop {
        let token = match iter.next() {
            Some(token) => token,
            None => return None,
        };
        let length = token.len();
        let token = convert(token, low, source, diagnostics);

        low = low.offset(length as _);

        match token {
            Some(token) => {
                return Some(token);
            }
            None => {}
        }
    })
}

macro_rules! literal {
    ($kind:expr, $str:expr) => {
        TokenKind::Literal({
            let str = { Symbol(STR_INTERNER.lock().intern($str)) };
            TokenLiteral::new($kind, str)
        })
    };
}

fn convert(
    token: LowToken,
    low: Pos,
    source: &Source,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Token> {
    let span = Span::new(low, low.offset(token.len() as _));

    check_low_token(&token, span, source, diagnostics);

    Some(Token::new(
        match token.kind() {
            LowTokenKind::Unknown | LowTokenKind::Whitespace | LowTokenKind::Comment => {
                return None;
            }
            LowTokenKind::OpenParen => TokenKind::OpenParen,
            LowTokenKind::CloseParen => TokenKind::CloseParen,
            LowTokenKind::OpenBrace => TokenKind::OpenBrace,
            LowTokenKind::CloseBrace => TokenKind::CloseBrace,
            LowTokenKind::OpenBracket => TokenKind::OpenBracket,
            LowTokenKind::CloseBracket => TokenKind::CloseBracket,
            LowTokenKind::Dot => TokenKind::Dot,
            LowTokenKind::Comma => TokenKind::Comma,
            LowTokenKind::Colon => TokenKind::Colon,
            LowTokenKind::Semicolon => TokenKind::Semicolon,
            LowTokenKind::Eq => TokenKind::Assign,
            LowTokenKind::Bang => TokenKind::Bang,
            LowTokenKind::Lt => TokenKind::Lt,
            LowTokenKind::Gt => TokenKind::Gt,
            LowTokenKind::Plus => TokenKind::Add,
            LowTokenKind::Minus => TokenKind::Sub,
            LowTokenKind::Star => TokenKind::Mul,
            LowTokenKind::Slash => TokenKind::Div,
            LowTokenKind::Percent => TokenKind::Mod,
            LowTokenKind::Or => TokenKind::BitOr,
            LowTokenKind::And => TokenKind::BitAnd,
            LowTokenKind::Caret => TokenKind::BitXor,
            LowTokenKind::Tilde => TokenKind::BitNot,
            LowTokenKind::Id => match source.slice(span) {
                "nil" => literal!(TokenLiteralKind::Nil, "nil"),
                "true" => literal!(TokenLiteralKind::Bool, "true"),
                "false" => literal!(TokenLiteralKind::Bool, "false"),
                "or" => TokenKind::LogOr,
                "and" => TokenKind::LogAnd,
                "not" => TokenKind::LogNot,
                id => TokenKind::id(id),
            },
            LowTokenKind::Literal(literal) => {
                let str = source.slice(span);

                match literal {
                    LowTokenLiteralKind::Integer => {
                        literal!(TokenLiteralKind::Integer, str)
                    }
                    LowTokenLiteralKind::Float => {
                        literal!(TokenLiteralKind::Float, str)
                    }
                    LowTokenLiteralKind::DoubleQuotedStr(..) => {
                        literal!(TokenLiteralKind::Str, str)
                    }
                }
            }
        },
        span,
    ))
}

fn check_low_token(
    token: &LowToken,
    span: Span,
    source: &Source,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match token.kind() {
        LowTokenKind::Unknown => diagnostics.push(Diagnostic::new(
            Level::Error,
            format!("unknown token '{}'", source.slice(span)),
            MultiSpan::with_spans(vec![(
                format!("'{}' is not allowed", source.slice(span)),
                Some(span),
            )]),
        )),
        LowTokenKind::Whitespace => {}
        LowTokenKind::Comment => {}
        LowTokenKind::OpenParen => {}
        LowTokenKind::CloseParen => {}
        LowTokenKind::OpenBrace => {}
        LowTokenKind::CloseBrace => {}
        LowTokenKind::OpenBracket => {}
        LowTokenKind::CloseBracket => {}
        LowTokenKind::Dot => {}
        LowTokenKind::Comma => {}
        LowTokenKind::Colon => {}
        LowTokenKind::Semicolon => {}
        LowTokenKind::Eq => {}
        LowTokenKind::Bang => {}
        LowTokenKind::Lt => {}
        LowTokenKind::Gt => {}
        LowTokenKind::Plus => {}
        LowTokenKind::Minus => {}
        LowTokenKind::Star => {}
        LowTokenKind::Slash => {}
        LowTokenKind::Percent => {}
        LowTokenKind::Or => {}
        LowTokenKind::And => {}
        LowTokenKind::Caret => {}
        LowTokenKind::Tilde => {}
        LowTokenKind::Id => {}
        LowTokenKind::Literal(literal) => match literal {
            LowTokenLiteralKind::Integer | LowTokenLiteralKind::Float => {}
            LowTokenLiteralKind::DoubleQuotedStr(str) => {
                if !str.terminated() {
                    diagnostics.push(Diagnostic::new(
                        Level::Error,
                        format!("double quoted literal is not closed"),
                        MultiSpan::with_spans(vec![
                            (format!("\" is missing"), Some(span)),
                            (format!("add \" at the end of the literal"), None),
                        ]),
                    ));
                }
            }
        },
    }
}

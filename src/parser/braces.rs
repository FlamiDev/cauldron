use crate::parser::parse_step::*;
use crate::{Spanned, Token};

#[macro_export]
macro_rules! parse_parens {
    ($variant:ident, $parser:expr, $trace:expr) => {
        move |arg: Option<&Spanned<Token>>| match arg {
            Some(v) => {
                match &v.value {
                    Token::$variant(t) => {
                    let mut t = t.iter();
                    while let Some(token) = t.next() {
                        match $parser(Some(token)) {
                            ParseStep::NotDone => {}
                            ParseStep::Done(value) => {
                                return if t.len() <= 0 {
                                    ParseStep::Done(value)
                                } else {
                                    error_partial(value, token.clone(), $trace)
                                }
                            }
                            ParseStep::Error {
                                partial,
                                mut errors
                            } => {
                                for error in errors.iter_mut() {
                                    error.trace.push($trace);
                                }
                                return ParseStep::Error { partial, errors }
                            }
                        }
                    }
                    match $parser(None) {
                        ParseStep::NotDone => error_eof($trace),
                        ParseStep::Error {
                            partial,
                            mut errors
                        } => {
                            for error in errors.iter_mut() {
                                error.trace.push($trace);
                            }
                            ParseStep::Error { partial, errors }
                        }
                        ParseStep::Done(value) => ParseStep::Done(value),
                    }
                }
                _ => error(v.clone(), $trace),
                }
            }
            None => error_eof($trace),
        }
    };
}

pub fn parentheses<T, F: Parser<T>>(mut parser: F) -> impl Parser<T> {
    parse_parens!(Parentheses, parser, TRACE_PARENTHESES)
}

pub fn braces<T, F: Parser<T>>(mut parser: F) -> impl Parser<T> {
    parse_parens!(Braces, parser, TRACE_BRACES)
}

pub fn brackets<T, F: Parser<T>>(mut parser: F) -> impl Parser<T> {
    parse_parens!(Brackets, parser, TRACE_BRACKETS)
}

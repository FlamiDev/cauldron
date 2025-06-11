use crate::parser::parse_step::*;
use crate::{Spanned, Token};

pub fn identifier(ident: &'static str) -> impl Parser<()> {
    move |arg: Option<&Spanned<Token>>| match arg {
        Some(v) => match &v.value {
            Token::Identifier(inner) => {
                if inner == ident {
                    done_span((), v)
                } else {
                    error(v.clone(), ident)
                }
            }
            _ => error(v.clone(), ident),
        },
        None => not_done(),
    }
}

pub fn symbol(ident: &'static str) -> impl Parser<()> {
    move |arg: Option<&Spanned<Token>>| match arg {
        Some(v) => match &v.value {
            Token::Symbol(inner) => {
                if inner == ident {
                    done_span((), v)
                } else {
                    error(v.clone(), ident)
                }
            }
            _ => error(v.clone(), ident),
        },
        None => not_done(),
    }
}

pub fn type_name() -> impl Parser<String> {
    move |arg: Option<&Spanned<Token>>| match arg {
        Some(v) => match &v.value {
            Token::Symbol(inner) => {
                let starts_upper = inner.starts_with(|c: char| c.is_ascii_uppercase());
                let contains_underscore = inner.contains('_');
                if starts_upper && !contains_underscore {
                    done_span(inner.clone(), v)
                } else {
                    error(v.clone(), TRACE_TYPE)
                }
            }
            _ => error(v.clone(), TRACE_TYPE),
        },
        None => not_done(),
    }
}

pub fn value_name() -> impl Parser<String> {
    move |arg: Option<&Spanned<Token>>| match arg {
        Some(v) => match &v.value {
            Token::Symbol(inner) => {
                let is_lower = !inner.contains(|c: char| !c.is_ascii_lowercase());
                if is_lower {
                    done_span(inner.clone(), v)
                } else {
                    error(v.clone(), TRACE_VALUE)
                }
            }
            _ => error(v.clone(), TRACE_VALUE),
        },
        None => not_done(),
    }
}

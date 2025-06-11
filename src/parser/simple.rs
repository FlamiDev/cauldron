use crate::parser::macros::match_token;
use crate::parser::parse_step::*;
use crate::{Spanned, Token};

pub fn int() -> impl Parser<i64> {
    match_token!(Integer, |i: &i64| *i, TRACE_INT)
}

pub fn float() -> impl Parser<f64> {
    match_token!(Float, |f: &f64| *f, TRACE_FLOAT)
}

pub fn boolean() -> impl Parser<bool> {
    match_token!(Boolean, |b: &bool| *b, TRACE_BOOLEAN)
}

pub fn string() -> impl Parser<String> {
    match_token!(StringLiteral, |s: &String| s.clone(), TRACE_STRING)
}

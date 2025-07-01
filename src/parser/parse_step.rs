use crate::{Spanned, Token};

pub const TRACE_INT: &str = "int";
pub const TRACE_FLOAT: &str = "float";
pub const TRACE_BOOLEAN: &str = "boolean";
pub const TRACE_STRING: &str = "string";
pub const TRACE_PARENTHESES: &str = "parentheses";
pub const TRACE_BRACKETS: &str = "brackets";
pub const TRACE_BRACES: &str = "braces";
pub const TRACE_TYPE: &str = "type_name";
pub const TRACE_VALUE: &str = "value_name";

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub on: Option<Spanned<Token>>,
    pub(crate) trace: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseStep<T> {
    NotDone,
    Done(Spanned<T>),
    Error {
        partial: Option<Spanned<T>>,
        errors: Vec<ParseError>,
    },
}

impl<T> ParseStep<T> {
    pub fn is_done(&self) -> bool {
        matches!(self, ParseStep::Done(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, ParseStep::Error { .. })
    }

    pub fn is_not_done(&self) -> bool {
        matches!(self, ParseStep::NotDone)
    }

    pub fn add_trace(mut self, trace: &'static str) -> Self {
        if let ParseStep::Error { errors, .. } = &mut self {
            for e in errors {
                e.trace.push(trace);
            }
        }
        self
    }
}

pub fn not_done<T>() -> ParseStep<T> {
    ParseStep::NotDone
}

pub fn done<T>(value: Spanned<T>) -> ParseStep<T> {
    ParseStep::Done(value)
}

pub fn done_span<T, S>(value: T, span: &Spanned<S>) -> ParseStep<T> {
    ParseStep::Done(span.with(value))
}

pub fn error<T>(on: Spanned<Token>, trace: &'static str) -> ParseStep<T> {
    ParseStep::Error {
        partial: None,
        errors: vec![ParseError {
            on: Some(on),
            trace: vec![trace],
        }],
    }
}

pub fn error_partial<T>(
    partial: Spanned<T>,
    on: Spanned<Token>,
    trace: &'static str,
) -> ParseStep<T> {
    ParseStep::Error {
        partial: Some(partial),
        errors: vec![ParseError {
            on: Some(on),
            trace: vec![trace],
        }],
    }
}

pub fn error_eof<T>(trace: &'static str) -> ParseStep<T> {
    ParseStep::Error {
        partial: None,
        errors: vec![ParseError {
            on: None,
            trace: vec![trace],
        }],
    }
}

pub trait Parser<T>: FnMut(Option<&Spanned<Token>>) -> ParseStep<T> {}
impl<T, F> Parser<T> for F where F: FnMut(Option<&Spanned<Token>>) -> ParseStep<T> {}

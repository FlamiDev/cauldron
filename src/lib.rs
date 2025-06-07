mod lexer;

pub use lexer::{lex, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}
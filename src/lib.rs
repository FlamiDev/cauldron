mod lexer;
mod parser;

pub use lexer::{lex, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl<T> Spanned<T> {
    pub fn new(value: T, line: usize, start: usize, end: usize) -> Spanned<T> {
        Spanned {
            value,
            line,
            start,
            end,
        }
    }
    
    pub fn with<U>(&self, value: U) -> Spanned<U> {
        Spanned {
            value,
            line: self.line,
            start: self.start,
            end: self.end,
        }
    }
    
    pub fn map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Spanned<U> {
        Spanned {
            value: f(&self.value),
            line: self.line,
            start: self.start,
            end: self.end,
        }
    }
}
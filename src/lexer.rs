mod group;
mod identifier;
mod lex_error;
mod number;
mod string;
mod symbol;

use crate::lexer::group::parse_group;
use crate::lexer::identifier::{is_ident_start, parse_identifier};
use crate::lexer::lex_error::LexError;
use crate::lexer::number::parse_number;
use crate::lexer::string::parse_string;
use crate::lexer::symbol::parse_symbol;
pub(crate) use crate::Spanned;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    StringLiteral(String),
    Identifier(String),
    Symbol(String),
    Parentheses(Vec<Spanned<Token>>),
    Brackets(Vec<Spanned<Token>>),
    Braces(Vec<Spanned<Token>>),
}

pub fn lex(source: &str) -> Result<Vec<Spanned<Token>>, LexError> {
    let chars = source.char_indices().peekable();
    lex_peekable(chars, 0)
}

fn lex_peekable(mut chars: Peekable<CharIndices>, mut line: usize) -> Result<Vec<Spanned<Token>>, LexError> {
    let mut tokens = Vec::new();
    let mut line_start = 0;

    while let Some((index, ch)) = chars.peek().copied() {
        // Update line count
        if ch == '\n' {
            chars.next();
            line += 1;
            line_start = index + 1;
            continue;
        }

        // Skip whitespace
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        // Skip comments
        if ch == '/' {
            if let Some((_, next_ch)) = chars.peek().copied() {
                if next_ch == '/' {
                    // Consume the comment until the end of the line
                    while let Some((i, comment_ch)) = chars.peek().copied() {
                        chars.next();
                        if comment_ch == '\n' {
                            line += 1;
                            line_start = i + 1;
                            break;
                        }
                    }
                    continue;
                }
            }
        }

        let column = index - line_start;

        match ch {
            c if c.is_ascii_digit() => {
                let (end, token) = parse_number(&mut chars, line, column)?;
                tokens.push(Spanned {
                    value: token,
                    line,
                    start: column,
                    end,
                });
            }

            '"' => {
                let (end, string) = parse_string(&mut chars, line, column)?;
                tokens.push(Spanned {
                    value: Token::StringLiteral(string),
                    line,
                    start: column,
                    end,
                });
            }

            '(' => {
                chars.next();
                let (end, contents) = parse_group(&mut chars, '(', ')', line, column)?;
                tokens.push(Spanned {
                    value: Token::Parentheses(contents),
                    line,
                    start: column,
                    end,
                });
            }

            '[' => {
                chars.next();
                let (end, contents) = parse_group(&mut chars, '[', ']', line, column)?;
                tokens.push(Spanned {
                    value: Token::Brackets(contents),
                    line,
                    start: column,
                    end,
                });
            }

            '{' => {
                chars.next();
                let (end, contents) = parse_group(&mut chars, '{', '}', line, column)?;
                tokens.push(Spanned {
                    value: Token::Braces(contents),
                    line,
                    start: column,
                    end,
                });
            }

            c if is_ident_start(c) => {
                let (end, ident) = parse_identifier(&mut chars, line, column);
                let token = match ident.as_str() {
                    "true" => Token::Boolean(true),
                    "false" => Token::Boolean(false),
                    _ => Token::Identifier(ident),
                };
                tokens.push(Spanned {
                    value: token,
                    line,
                    start: column,
                    end,
                });
            }

            _ => {
                let (end, symbol) = parse_symbol(&mut chars, line, column);
                tokens.push(Spanned {
                    value: Token::Symbol(symbol),
                    line,
                    start: column,
                    end,
                });
            }
        }
    }

    Ok(tokens)
}

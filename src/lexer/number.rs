use crate::lexer::{LexError, Token};
use std::iter::Peekable;
use std::str::CharIndices;

pub fn lex_number(
    chars: &mut Peekable<CharIndices<'_>>,
    line: usize,
    column: usize,
) -> Result<(usize, Token), LexError> {
    let start = column;
    let mut end = start;
    let mut dot_seen = false;
    let mut number_str = String::new();

    while let Some(&(_, ch)) = chars.peek() {
        if ch == '.' {
            if dot_seen {
                return Err(LexError::InvalidNumber {
                    line,
                    position: end,
                });
            }
            dot_seen = true;
            end += ch.len_utf8(); // Adjust for UTF-8 multi-byte characters
            number_str.push(ch);
            chars.next();
        } else if ch.is_ascii_digit() {
            end += ch.len_utf8(); // Adjust for UTF-8 multi-byte characters
            number_str.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    if dot_seen {
        number_str
            .parse::<f64>()
            .map(|f| (end, Token::Float(f)))
            .map_err(|_| LexError::InvalidNumber {
                line,
                position: start,
            })
    } else {
        number_str
            .parse::<i64>()
            .map(|i| (end, Token::Integer(i)))
            .map_err(|_| LexError::InvalidNumber {
                line,
                position: start,
            })
    }
}

use crate::lexer::LexError;
use std::iter::Peekable;
use std::str::CharIndices;

pub fn parse_string(
    chars: &mut Peekable<CharIndices<'_>>,
    line: usize,
    column: usize,
) -> Result<(usize, String), LexError> {
    let start = column;
    let mut result = String::new();
    let mut end = column;
    chars.next(); // consume '"'

    while let Some(&(_i, ch)) = chars.peek() {
        if ch == '"' {
            end += ch.len_utf8();
            chars.next();
            return Ok((end, result));
        } else if ch == '\n' {
            // Strings cannot span multiple lines, so this is an error
            return Err(LexError::UnterminatedString {
                line,
                position: start,
            });
        } else {
            result.push(ch);
            end += ch.len_utf8();
            chars.next();
        }
    }

    Err(LexError::UnterminatedString {
        line,
        position: start,
    })
}

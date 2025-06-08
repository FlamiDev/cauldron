use std::iter::Peekable;
use std::str::CharIndices;

pub fn lex_identifier(
    chars: &mut Peekable<CharIndices<'_>>,
    _line: usize,
    column: usize,
) -> (usize, String) {
    let mut ident = String::new();
    let mut end = column;
    while let Some(&(_, ch)) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            ident.push(ch);
            end += ch.len_utf8();
            chars.next();
        } else {
            break;
        }
    }
    (end, ident)
}

pub fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

use std::iter::Peekable;
use std::str::CharIndices;

pub fn lex_symbol(
    chars: &mut Peekable<CharIndices<'_>>,
    _line: usize,
    column: usize,
) -> (usize, String) {
    let mut symbol = String::new();
    let mut end = column;
    while let Some(&(_, ch)) = chars.peek() {
        if ch.is_alphanumeric() || ch.is_whitespace() || "[](){}".contains(ch) || ch == '"' {
            break;
        }
        symbol.push(ch);
        end += ch.len_utf8();
        chars.next();
    }
    (end, symbol)
}

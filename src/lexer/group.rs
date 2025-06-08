use crate::lexer::{lex_peekable, LexError, Token};
use crate::Spanned;
use std::iter::Peekable;
use std::str::CharIndices;

pub fn lex_group(
    chars: &mut Peekable<CharIndices<'_>>,
    start_delim: char,
    end_delim: char,
    line: usize,
    column: usize,
) -> Result<(usize, Vec<Spanned<Token>>), LexError> {
    let mut depth = 1;
    let mut group_content = String::new();
    let start = column;
    let mut end = column;
    let mut last_line = line;
    let mut last_col = column;

    while let Some(&(_, ch)) = chars.peek() {
        chars.next();
        if ch == '\n' {
            last_line += 1;
            last_col = 0;
            group_content.push(ch);
            continue;
        }
        last_col += ch.len_utf8();
        if ch == start_delim {
            depth += 1;
            group_content.push(ch);
        } else if ch == end_delim {
            depth -= 1;
            if depth == 0 {
                end = last_col;
                break;
            } else {
                group_content.push(ch);
            }
        } else {
            group_content.push(ch);
        }
    }

    if depth != 0 {
        return Err(LexError::UnclosedGroup {
            line: last_line,
            position: start,
            delimiter: end_delim,
        });
    }

    Ok((
        end,
        lex_peekable(group_content.char_indices().peekable(), line)?,
    ))
}

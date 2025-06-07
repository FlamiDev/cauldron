#[derive(Debug)]
pub enum LexError {
    UnterminatedString {
        line: usize,
        position: usize,
    },
    UnexpectedChar {
        line: usize,
        position: usize,
        char: char,
    },
    UnclosedGroup {
        line: usize,
        position: usize,
        delimiter: char,
    },
    InvalidNumber {
        line: usize,
        position: usize,
    },
}

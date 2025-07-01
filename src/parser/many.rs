use crate::parser::parse_step::*;
use crate::{Spanned, Token};
use std::vec::Vec;

pub fn many1<T: Clone, F: Parser<T>>(mut parser: F) -> impl Parser<Vec<T>> {
    move |input: Option<&Spanned<Token>>| {
        let result = many0(&mut parser)(input);

        match result {
            ParseStep::Done(spanned) if spanned.value.is_empty() => {
                // If many0 returns an empty result, return an error
                ParseStep::Error {
                    partial: None,
                    errors: vec![ParseError {
                        on: input.cloned(),
                        trace: vec!["Expected at least one match"],
                    }],
                }
            }
            _ => result,
        }
    }
}

pub fn many0<T: Clone, F: Parser<T>>(mut parser: F) -> impl Parser<Vec<T>> {
    let mut results = Vec::new();
    let mut first_token = true;
    let mut line = 0;
    let mut start = 0;
    let mut end = 0;
    move |input: Option<&Spanned<Token>>| {
        if let Some(spanned) = input {
            if first_token {
                line = spanned.line;
                start = spanned.start;
                first_token = false;
            }
            end = spanned.end;
        }
        match parser(input) {
            ParseStep::Done(spanned) => {
                results.push(spanned.value);
            }
            ParseStep::NotDone => return ParseStep::NotDone,
            ParseStep::Error { partial, errors } => {
                if let Some(partial_input) = partial {
                    results.push(partial_input.value);
                }
                return ParseStep::Error {
                    partial: Some(Spanned::new(results.clone(), line, start, end)),
                    errors,
                };
            }
        }
        if input.is_some() {
            return ParseStep::NotDone;
        }
        let results_clone = results.clone();
        results.clear();
        first_token = true;
        ParseStep::Done(Spanned::new(results_clone, line, start, end))
    }
}

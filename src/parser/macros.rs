macro_rules! match_token {
    ($variant:ident, $then:expr, $trace:expr) => {
        move |arg: Option<&Spanned<Token>>| match arg {
            Some(v) => match &v.value {
                Token::$variant(inner) => done_span($then(inner), v),
                _ => error(v.clone(), $trace),
            },
            None => not_done(),
        }
    };
}

pub(crate) use match_token;
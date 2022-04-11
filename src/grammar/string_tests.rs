#[cfg(test)]
use super::{string::string, token::Token};
#[cfg(test)]
use nom::{Err, IResult};

#[test]
fn should_be_valid_double_quoted_string() {
    let input = "\"hello world\"";

    let res = string(input);

    assert_eq!(Ok(("", Token::String("hello world".to_owned()))), res)
}

#[test]
fn should_be_valid_single_quoted_string() {
    let input = "\'hello world\'";

    let res = string(input);

    assert_eq!(Ok(("", Token::String("hello world".to_owned()))), res)
}

#[test]
fn error_on_missing_quote() {
    let input = "\"hello world";

    let res = string(input);

    assert_eq!(IResult::Err(Err::Error(())), res)
}

#[test]
fn error_on_newline() {
    let input = "\"hello\nworld\"";

    let res = string(input);

    assert_eq!(IResult::Err(Err::Error(())), res)
}

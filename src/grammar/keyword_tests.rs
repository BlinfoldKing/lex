#[cfg(test)]
use super::{keyword::keyword, token::Token};
#[cfg(test)]
use nom::Err;

#[test]
fn should_be_valid_keyword() {
    let input = ".hello_world";

    let res = keyword(input);

    assert_eq!(Ok(("", Token::Keyword("hello_world"))), res)
}

#[test]
fn error_missing_dot() {
    let input = "_hello_world";

    let res = keyword(input);

    assert_eq!(Err(Err::Error(())), res)
}

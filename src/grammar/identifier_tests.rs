#[cfg(test)]
use super::{identifier::identifier, token::Token};
#[cfg(test)]
use nom::Err;

#[test]
fn should_be_valid_identifier() {
    let input = ".hello_world";

    let res = identifier(input);

    assert_eq!(Ok(("", Token::Identifier("hello_world".to_owned()))), res)
}

#[test]
fn error_missing_dot() {
    let input = "_hello_world";

    let res = identifier(input);

    assert_eq!(Err(Err::Error(())), res)
}

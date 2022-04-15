#[cfg(test)]
use super::{operator::operator, token::Token};
#[cfg(test)]
use nom::Err;

#[test]
fn should_be_valid_operator() {
    let input = "+";

    let res = operator(input);

    assert_eq!(Ok(("", Token::Operator("+".to_owned()))), res)
}

#[test]
fn error_invalid_symbol() {
    let input = "@";

    let res = operator(input);

    assert_eq!(Err(Err::Error(())), res)
}

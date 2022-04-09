#[cfg(test)]
use super::{number::number, token::Token};

#[test]
fn should_be_valid_number() {
    let input = "3.14";

    let res = number(input);

    assert_eq!(Ok(("", Token::Number(3.14))), res)
}

#[cfg(test)]
use super::{token::Token, variable::variable};

#[test]
fn should_be_valid_keyword() {
    let input = "Hello_world";

    let res = variable(input);

    assert_eq!(Ok(("", Token::Variable("Hello_world"))), res)
}

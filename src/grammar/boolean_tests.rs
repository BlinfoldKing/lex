#[cfg(test)]
use super::{boolean::boolean, token::Token};

#[test]
fn should_be_true() {
    let input = "true";

    let res = boolean(input);

    assert_eq!(Ok(("", Token::Boolean(true))), res)
}

#[test]
fn should_be_false() {
    let input = "false";

    let res = boolean(input);

    assert_eq!(Ok(("", Token::Boolean(false))), res)
}

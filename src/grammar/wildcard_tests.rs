#[cfg(test)]
use super::{token::Token, wildcard::wildcard};

#[test]
fn should_be_valid_keyword() {
    let input = "_Hello_world";

    let res = wildcard(input);

    assert_eq!(Ok(("", Token::Wildcard("Hello_world", None))), res)
}

#[cfg(test)]
use super::{token::Token, variable::variable};

#[test]
fn should_be_valid_keyword() {
    let input = "HelloWorld";

    let res = variable(input);

    assert_eq!(
        Ok(("", Token::Variable("HelloWorld".to_owned(), None))),
        res
    )
}

#[cfg(test)]
use super::{inline_comment::inline_comment, token::Token};

#[test]
fn should_be_able_to_end_with_endline() {
    let comment = "//hello world\n";

    let res = inline_comment(comment);

    assert_eq!(Ok(("\n", Token::Comment("hello world"))), res)
}

#[test]
fn should_be_able_to_end_with_eol() {
    let comment = "//hello world";

    let res = inline_comment(comment);

    assert_eq!(Ok(("", Token::Comment("hello world"))), res)
}

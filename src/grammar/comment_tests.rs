#[cfg(test)]
use super::{comment::comment, token::Token};
#[cfg(test)]
use nom::{Err, IResult};

#[test]
fn should_be_able_to_comment() {
    let input = "/*hello world*/";

    let res = comment(input);

    assert_eq!(Ok(("", Token::Comment("hello world"))), res)
}

#[test]
fn should_be_able_to_comment_multiline() {
    let input = r#"/*
hello world
*/"#;

    let res = comment(input);

    assert_eq!(Ok(("", Token::Comment("\nhello world\n"))), res)
}

#[test]
fn error_with_missing_pair() {
    let input = "/*hello world";

    let res = comment(input);

    assert_eq!(IResult::Err(Err::Error(())), res)
}

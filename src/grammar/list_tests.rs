#[cfg(test)]
use super::{list::list, token::Token};
#[cfg(test)]
use nom::Err;

#[test]
fn should_be_valid_list() {
    let input = "(1 1 1)";

    let res = list(input);

    assert_eq!(
        Ok((
            "",
            Token::List(vec![
                Token::Number(1 as f64),
                Token::Number(1 as f64),
                Token::Number(1 as f64),
            ])
        )),
        res
    )
}

#[test]
fn should_be_valid_list_of_mixed_type() {
    let input = "(\"hello world\" 1 1)";

    let res = list(input);

    assert_eq!(
        Ok((
            "",
            Token::List(vec![
                Token::String("hello world"),
                Token::Number(1 as f64),
                Token::Number(1 as f64),
            ])
        )),
        res
    )
}

#[test]
fn should_be_valid_empty_list() {
    let input = "()";

    let res = list(input);

    assert_eq!(Ok(("", Token::List(vec![]))), res)
}

#[test]
fn should_be_valid_nested_list() {
    let input = "(\"hello world\" (1) 1)";

    let res = list(input);

    assert_eq!(
        Ok((
            "",
            Token::List(vec![
                Token::String("hello world"),
                Token::List(vec![Token::Number(1 as f64)]),
                Token::Number(1 as f64),
            ])
        )),
        res
    )
}

#[test]
fn error_missing_parentheses() {
    let input = "(\"hello world\" ";

    let res = list(input);

    assert_eq!(Err(Err::Error(())), res)
}

#[cfg(test)]
use super::{atom::atom, token::Token};
#[cfg(test)]
use nom::Err;

#[test]
fn should_be_valid_with_underscore() {
    let input = "hello_world";

    let res = atom(input);

    assert_eq!(Ok(("", Token::Atom("hello_world"))), res)
}

#[test]
fn error_start_with_capital() {
    let input = "Hello-world";

    let res = atom(input);

    assert_eq!(Err(Err::Error(())), res)
}

#[test]
fn error_start_with_number() {
    let input = "1hello-world";

    let res = atom(input);

    assert_eq!(Err(Err::Error(())), res)
}

#[test]
fn error_start_with_symbol() {
    let input = ".hello-world";

    let res = atom(input);

    assert_eq!(Err(Err::Error(())), res)
}

use super::token::Token;
use nom::{bytes::complete::is_a, character::complete::one_of, IResult};

pub fn atom(input: &str) -> IResult<&str, Token, ()> {
    let (_, _) = one_of("abcdefghijklmnopqrstuvwxyz")(input)?;

    let (input, value) =
        is_a("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")(input)?;

    Ok((&input, Token::Atom(value.to_owned())))
}

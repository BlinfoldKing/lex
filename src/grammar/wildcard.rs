use super::token::Token;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::one_of,
    sequence::pair,
    IResult,
};

pub fn wildcard(input: &str) -> IResult<&str, Token, ()> {
    let (input, (_, value)) = pair(
        tag("_"),
        is_a("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"),
    )(input)?;

    let (_, _) = one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(value)?;

    Ok((&input, Token::Wildcard(value)))
}

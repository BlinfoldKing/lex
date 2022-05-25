use super::token::Token;
use nom::{
    bytes::complete::{is_a, tag},
    sequence::pair,
    IResult,
};

pub fn identifier(input: &str) -> IResult<&str, Token, ()> {
    let (input, (_, value)) = pair(
        tag("."),
        is_a("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"),
    )(input)?;

    Ok((&input, Token::Identifier(value.to_owned())))
}

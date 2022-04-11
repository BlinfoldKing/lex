use super::token::Token;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    sequence::tuple,
    IResult,
};

pub fn string(input: &str) -> IResult<&str, Token, ()> {
    let (input, (_, value, _)) = alt((
        tuple((tag("\""), is_not("\"\n\r"), tag("\""))),
        tuple((tag("\'"), is_not("\'\n\r"), tag("\'"))),
    ))(input)?;

    Ok((&input, Token::String(value.to_owned())))
}

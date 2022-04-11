use super::token::Token;
use nom::{bytes::complete::is_a, IResult};

pub fn whitespace(input: &str) -> IResult<&str, Token, ()> {
    let (input, _value) = is_a("\t\n ")(input)?;

    Ok((input, Token::Whitespace))
}

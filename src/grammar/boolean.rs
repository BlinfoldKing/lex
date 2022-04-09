use super::token::Token;
use nom::{branch::alt, bytes::complete::tag, IResult};

pub fn boolean(input: &str) -> IResult<&str, Token, ()> {
    let (input, value) = alt((tag("true"), tag("false")))(input)?;

    Ok((&input, Token::Boolean(value == "true")))
}

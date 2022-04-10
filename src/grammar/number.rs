use super::token::Token;
use nom::{number::complete::double, IResult};

pub fn number(input: &str) -> IResult<&str, Token, ()> {
    let (input, value) = double(input)?;

    Ok((input, Token::Number(value)))
}

use super::token::Token;
use nom::{branch::alt, bytes::complete::tag, IResult};

pub fn operator(input: &str) -> IResult<&str, Token, ()> {
    let (input, value) = alt((
        tag("!="),
        tag("="),
        tag(">"),
        tag("<"),
        tag(">="),
        tag("<="),
        tag(":="),
        tag("!"),
        tag("-"),
        tag("+"),
        tag("/"),
        tag("*"),
        tag("?"),
        tag("!"),
    ))(input)?;

    Ok((&input, Token::Operator(value)))
}

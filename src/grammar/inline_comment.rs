use super::token::Token;
use nom::{
    bytes::complete::{is_not, tag},
    sequence::pair,
    IResult,
};

pub fn inline_comment(input: &str) -> IResult<&str, Token, ()> {
    let (input, (_, value)) = pair(tag("//"), is_not("\n\r"))(input)?;

    Ok((&input, Token::Comment))
}

use super::token::Token;
use nom::{
    bytes::complete::{tag, take_until},
    sequence::tuple,
    IResult,
};

pub fn comment(input: &str) -> IResult<&str, Token, ()> {
    let (input, (_, _value, _)) = tuple((tag("/*"), take_until("*/"), tag("*/")))(input)?;

    Ok((&input, Token::Comment))
}

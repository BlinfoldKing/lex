use super::token::Token;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    multi::many0,
    sequence::tuple,
    IResult,
};

pub fn string(input: &str) -> IResult<&str, Token, ()> {
    let (input, (_, value, _)) = alt((
        tuple((tag("\""), many0(is_not("\"\n\r")), tag("\""))),
        tuple((tag("\'"), many0(is_not("\'\n\r")), tag("\'"))),
    ))(input)?;

    let res: String = value.into_iter().fold("".to_owned(), |acc, item| {
        let mut ret = acc;
        ret += item;

        ret
    });

    Ok((&input, Token::String(res)))
}

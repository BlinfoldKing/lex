use super::token::Token;
use nom::{branch::alt, bytes::complete::tag, sequence::pair, IResult};

pub fn keyword<'a>(word: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, Token, ()> {
    move |input: &str| {
        let (input, (_, value)) = pair(tag("@"), tag(word))(input)?;

        Ok((&input, Token::Keyword(value.to_owned())))
    }
}

pub fn valid_keyword(input: &str) -> IResult<&str, Token, ()> {
    let res = alt((
        keyword("include"),
        keyword("return"),
        keyword("def"),
        keyword("dec"),
        keyword("do"),
        keyword("and"),
        keyword("or"),
        keyword("div"),
    ))(input)?;

    Ok(res)
}

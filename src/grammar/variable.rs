use super::token::Token;
use nom::{
    character::complete::one_of,
    combinator::{opt, recognize},
    multi::many1,
    sequence::tuple,
    IResult,
};

pub fn variable(input: &str) -> IResult<&str, Token, ()> {
    let (input, value) = recognize(tuple((
        one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        opt(many1(one_of(
            "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890",
        ))),
    )))(input)?;

    Ok((&input, Token::Variable(value.to_owned(), None)))
}

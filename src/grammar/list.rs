use super::atom::atom;
use super::boolean::boolean;
use super::keyword::keyword;
use super::number::number;
use super::operator::operator;
use super::string::string;
use super::token::Token;
use super::variable::variable;
use super::wildcard::wildcard;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    Err, IResult,
};

fn whitespace(input: &str) -> IResult<&str, char, ()> {
    let (input, value) = one_of("\t\n ")(input)?;

    Ok((input, value))
}

pub fn list(input: &str) -> IResult<&str, Token, ()> {
    if input.len() < 2 {
        return Err(Err::Error(()));
    }

    if input.len() == 2 {
        let (input, _) = tuple((tag("("), tag(")")))(input)?;
        return Ok((&input, Token::List(vec![])));
    }

    let element = separated_list1(
        many1(whitespace),
        alt((
            atom, boolean, keyword, list, number, operator, string, variable, wildcard,
        )),
    );

    let (input, (_, _, value, _, _)) = tuple((
        tag("("),
        many0(whitespace),
        element,
        many0(whitespace),
        tag(")"),
    ))(input)?;

    // ignore comment
    let result: Vec<Token> = value
        .clone()
        .into_iter()
        .filter(|e| *e != Token::Comment)
        .collect();

    Ok((&input, Token::List(result)))
}

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
    branch::alt, bytes::complete::tag, character::complete::space1, multi::separated_list1,
    sequence::tuple, Err, IResult,
};

pub fn list(input: &str) -> IResult<&str, Token, ()> {
    if input.len() < 2 {
        return Err(Err::Error(()));
    }

    if input.len() == 2 {
        let (input, _) = tuple((tag("("), tag(")")))(input)?;
        return Ok((&input, Token::List(vec![])));
    }

    let (input, (_, value, _)) = tuple((
        tag("("),
        separated_list1(
            space1,
            alt((
                atom, boolean, keyword, list, number, operator, string, variable, wildcard,
            )),
        ),
        tag(")"),
    ))(input)?;

    Ok((&input, Token::List(value)))
}

use super::keyword::keyword;
use super::token::{token, Token};
use super::whitespace::whitespace;
use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    Err, IResult,
};

pub fn l_parent(input: &str) -> IResult<&str, Token, ()> {
    let (input, _) = tag("(")(input)?;

    Ok((input, Token::LParent))
}

pub fn r_parent(input: &str) -> IResult<&str, Token, ()> {
    let (input, _) = tag(")")(input)?;

    Ok((input, Token::RParent))
}

pub fn list(input: &str) -> IResult<&str, Token, ()> {
    if input.len() < 2 {
        return Err(Err::Error(()));
    }

    if input.len() == 2 {
        let (input, _) = tuple((tag("("), tag(")")))(input)?;
        return Ok((&input, Token::List(vec![])));
    }

    let element = separated_list1(many1(whitespace), token);
    let with_parent = tuple((
        l_parent,
        many0(whitespace),
        element,
        many0(whitespace),
        r_parent,
    ));

    let element1 = separated_list1(many1(whitespace), token);
    let with_keyword = tuple((
        keyword("begin"),
        many0(whitespace),
        element1,
        many0(whitespace),
        keyword("end"),
    ));

    let (input, (_, _, value, _, _)) = alt((with_parent, with_keyword))(input)?;

    // ignore comment
    let result: Vec<Token> = value
        .clone()
        .into_iter()
        .filter(|e| *e != Token::Comment)
        .collect();

    // check for list variant
    let res = match &result[..] {
        [Token::Keyword(key), a, b] => {
            if key == "def" {
                Token::Definition(Box::new(a.clone()), Box::new(b.clone()))
            } else {
                Token::List(result)
            }
        }
        [Token::Keyword(key), a] => {
            if key == "dec" {
                Token::Definition(Box::new(a.clone()), Box::new(Token::_true()))
            } else {
                Token::List(result)
            }
        }
        _ => Token::List(result),
    };

    Ok((&input, res))
}

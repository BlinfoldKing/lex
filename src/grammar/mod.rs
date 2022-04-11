/*
 * TODO: write custom parser,
 * using nom for quicker prototyping
 */
pub mod token;

pub mod atom;
pub mod atom_tests;
pub mod boolean;
pub mod boolean_tests;
pub mod comment;
pub mod comment_tests;
pub mod inline_comment;
pub mod inline_comment_tests;
pub mod keyword;
pub mod keyword_tests;
pub mod list;
pub mod list_tests;
pub mod number;
pub mod number_tests;
pub mod operator;
pub mod operator_tests;
pub mod string;
pub mod string_tests;
pub mod variable;
pub mod variable_tests;
pub mod whitespace;
pub mod wildcard;
pub mod wildcard_tests;

use nom::{branch::alt, multi::many1, IResult};
use token::Token;

fn document(input: &str) -> IResult<&str, Token, ()> {
    let (input, value) = many1(alt((
        comment::comment,
        inline_comment::inline_comment,
        list::list,
        whitespace::whitespace,
    )))(input)?;

    Ok((
        input,
        Token::List(
            value
                .into_iter()
                .filter(|token| *token != Token::Whitespace || *token != Token::Comment)
                .collect(),
        ),
    ))
}

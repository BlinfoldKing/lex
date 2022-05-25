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
pub mod identifier;
pub mod identifier_tests;
pub mod inline_comment;
pub mod inline_comment_tests;
pub mod keyword;
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

use atom::atom;
use boolean::boolean;
use identifier::identifier;
use keyword::keyword;
use list::list;
use nom::{
    branch::alt,
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};
use number::number;
use operator::operator;
use string::string;
use token::Token;
use variable::variable;
use whitespace::whitespace;
use wildcard::wildcard;

pub fn repl_line(input: &str) -> IResult<&str, Token, ()> {
    let value = alt((
        boolean, string, atom, identifier, list, number, operator, variable, wildcard,
    ));
    let (inp, (_, value, _)) = tuple((many0(whitespace), value, many0(whitespace)))(input)?;

    Ok((inp, value))
}

pub fn document(input: &str) -> IResult<&str, Token, ()> {
    let ignore1 = alt((
        comment::comment,
        whitespace::whitespace,
        inline_comment::inline_comment,
    ));
    let ignore2 = alt((
        comment::comment,
        whitespace::whitespace,
        inline_comment::inline_comment,
    ));

    let list = many1(alt((
        comment::comment,
        inline_comment::inline_comment,
        token::token,
        whitespace::whitespace,
    )));

    let (input, (_, _, _, name, value)) = tuple((
        many0(ignore1),
        keyword("document"),
        many0(ignore2),
        alt((string::string, atom::atom)),
        list,
    ))(input)?;

    let value: Vec<Token> = value
        .into_iter()
        .filter(|token| *token != Token::Whitespace && *token != Token::Comment)
        .collect();

    Ok((
        input,
        Token::Document(Box::new(name), Box::new(Token::List(value))),
    ))
}

#[test]
fn should_be_able_to_parse_document() {
    let doc = r#"
//some comment
@document "hello_world"
//some comment
(hello world)
//some comment
(hello world)
//some comment
"#;

    let (_, res) = document(doc).unwrap();

    assert_eq!(
        Token::Document(
            Box::new(Token::String("hello world".to_owned())),
            Box::new(Token::List(vec![
                Token::List(vec![
                    Token::Atom("hello".to_owned()),
                    Token::Atom("world".to_owned())
                ]),
                Token::List(vec![
                    Token::Atom("hello".to_owned()),
                    Token::Atom("world".to_owned())
                ])
            ]))
        ),
        res
    )
}

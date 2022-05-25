use crate::utils::posibility::Posibility;

#[derive(Clone, Debug)]
pub enum Token {
    // language features
    Document(Box<Token>, Box<Token>),
    LParent,
    RParent,
    Whitespace,
    Comment,

    // types
    Value,
    String(String),
    Number(f64),
    Atom(String),
    Operator(String),
    Keyword(String),
    Identifier(String),
    Variable(String, Option<Posibility<Token>>),
    Wildcard(String, Option<Posibility<Token>>),
    Boolean(bool),

    // list variant
    List(Vec<Token>),
    Definition(Box<Token>, Box<Token>),
    UnaryOperation(Box<Token>, Box<Token>),
    BinaryOperation(Box<Token>, Box<Token>, Box<Token>),
    ListOperation(Box<Token>, Vec<Token>),
}

impl Token {
    pub fn _true() -> Self {
        Token::Boolean(true)
    }

    pub fn _false() -> Self {
        Token::Boolean(false)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Document(_, a), Token::Document(_, b)) => a == b,
            (Token::Document(_, _), _) | (_, Token::Document(_, _)) => false,
            (Token::Definition(a, _), Token::Definition(b, _)) => a == b,

            (Token::Value, _) | (_, Token::Value) => true,

            (Token::Comment, Token::Comment) | (Token::Whitespace, Token::Whitespace) => true,
            (_, Token::Comment) | (_, Token::Whitespace) => false,

            (Token::Operator(a), Token::Operator(b))
            | (Token::Keyword(a), Token::Keyword(b))
            | (Token::Identifier(a), Token::Identifier(b)) => a == b,
            (Token::Operator(_), _) | (_, Token::Operator(_)) => false,

            (Token::Identifier(_), _) | (_, Token::Identifier(_)) => false,
            (Token::Keyword(_), _) | (_, Token::Keyword(_)) => false,

            (Token::Number(a), Token::Number(b)) => a == b,
            (Token::Boolean(a), Token::Boolean(b)) => a == b,
            (Token::Atom(a), Token::Atom(b)) => a == b,
            (Token::String(a), Token::String(b)) => a == b,
            (Token::List(a), Token::List(b)) => a
                .into_iter()
                .zip(b.into_iter())
                .map(|(x, y)| x == y)
                .fold(true, |acc, val| acc && val),

            (Token::Variable(_, None), Token::Variable(_, None)) => true,
            (_, Token::Variable(_, None)) | (Token::Variable(_, None), _) => true,
            (a, Token::Variable(_, Some(b))) | (Token::Variable(_, Some(b)), a) => *b == *a,

            (Token::Wildcard(_, None), Token::Wildcard(_, None)) => true,
            (_, Token::Wildcard(_, None)) | (Token::Wildcard(_, None), _) => true,
            (a, Token::Wildcard(_, Some(b))) | (Token::Wildcard(_, Some(b)), a) => *b == *a,

            _ => false,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::List(list) => {
                write!(f, "(").unwrap();
                for (i, token) in list.into_iter().enumerate() {
                    if i > 0 {
                        write!(f, " ").unwrap();
                    }
                    token.fmt(f).unwrap();
                }
                write!(f, ")")
            }
            Token::Boolean(b) => write!(f, "{}", b),
            Token::Variable(str, None) => write!(f, "{}", str),
            Token::Variable(str, Some(value)) => {
                write!(f, "[").unwrap();
                write!(f, "{}:{:?}", str, *value).unwrap();
                write!(f, "]")
            }
            Token::Wildcard(str, _) => write!(f, "_{}", str),
            Token::Atom(str) | Token::Operator(str) => write!(f, "{}", str),
            Token::Keyword(str) => write!(f, "@{}", str),
            Token::Identifier(str) => write!(f, ".{}", str),
            Token::String(str) => write!(f, "\"{}\"", str),
            Token::Number(n) => write!(f, "{}", n),
            Token::Value => write!(f, "$"),
            Token::Whitespace => write!(f, "{{:whitespace}}"),
            Token::Comment => write!(f, "{{:comment}}"),
            Token::Document(name, content) => write!(f, "@document {} {}", name, content),
            Token::Definition(def, res) => write!(f, "(@def {} {})", def, res),
            _ => write!(f, "{{:invalid}}"),
        }
    }
}

use super::atom::atom;
use super::boolean::boolean;
use super::identifier::identifier;
use super::keyword::valid_keyword;
use super::list::list;
use super::number::number;
use super::operator::operator;
use super::string::string;
use super::variable::variable;
use super::wildcard::wildcard;
use nom::{branch::alt, IResult};

pub fn token(input: &str) -> IResult<&str, Token, ()> {
    let res = alt((
        list,
        boolean,
        string,
        variable,
        atom,
        valid_keyword,
        identifier,
        number,
        operator,
        variable,
        wildcard,
    ))(input)?;

    Ok(res)
}

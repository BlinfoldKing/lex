#[derive(Clone, Debug)]
pub enum Token {
    Comment,
    Value,
    String(String),
    Number(f64),
    Atom(String),
    Operator(String),
    Keyword(String),
    Variable(String, Option<Box<Token>>),
    Wildcard(String, Option<Box<Token>>),
    Boolean(bool),
    List(Vec<Token>),
    Whitespace,
    // Document(name: [Atom | String], do: List)
    Document(Box<Token>, Box<Token>),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Document(_, a), Token::Document(_, b)) => a == b,
            (Token::Value, _) | (_, Token::Value) => true,
            (Token::Wildcard(_, None), Token::Wildcard(_, None))
            | (Token::Variable(_, None), Token::Variable(_, None)) => true,
            (Token::Wildcard(_, Some(b)), Token::Variable(_, Some(a)))
            | (Token::Variable(_, Some(b)), Token::Wildcard(_, Some(a))) => a == b,
            (Token::Variable(_, Some(a)), b) | (Token::Wildcard(_, Some(a)), b) => &**a == b,
            (b, Token::Variable(_, Some(a))) | (b, Token::Wildcard(_, Some(a))) => &**a == b,
            (Token::Comment, Token::Comment) | (Token::Whitespace, Token::Whitespace) => true,
            (Token::String(a), Token::String(b)) => a == b,
            (Token::Atom(a), Token::Atom(b))
            | (Token::Operator(a), Token::Operator(b))
            | (Token::Keyword(a), Token::Keyword(b)) => a == b,
            (Token::Boolean(a), Token::Boolean(b)) => a == b,
            (Token::List(a), Token::List(b)) => a == b,
            (Token::Variable(_, _), Token::Comment) | (Token::Comment, Token::Variable(_, _)) => {
                false
            }
            (Token::Number(a), Token::Number(b)) => a == b,
            (Token::Wildcard(_, _), Token::Comment) | (Token::Comment, Token::Wildcard(_, _)) => {
                false
            }
            (Token::Variable(_, None), _) | (_, Token::Variable(_, None)) => true,
            (Token::Wildcard(_, None), _) | (_, Token::Wildcard(_, None)) => true,
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
                write!(f, "{}:{}", str, **value).unwrap();
                write!(f, "]")
            }
            Token::Wildcard(str, _) => write!(f, "_{}", str),
            Token::Atom(str) | Token::Operator(str) => write!(f, "{}", str),
            Token::Keyword(str) => write!(f, ".{}", str),
            Token::String(str) => write!(f, "\"{}\"", str),
            Token::Number(n) => write!(f, "{}", n),
            Token::Value => write!(f, "{{:value}}"),
            Token::Whitespace => write!(f, "{{:whitespace}}"),
            Token::Comment => write!(f, "{{:comment}}"),
            Token::Document(name, content) => write!(f, ".document {} {}", name, content),
            // _ => write!(f, "{{:invalid}}"),
        }
    }
}

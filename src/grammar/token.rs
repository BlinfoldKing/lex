#[derive(Clone)]
pub enum Token<'a> {
    Comment,
    Value,
    String(String),
    Number(f64),
    Atom(&'a str),
    Operator(&'a str),
    Keyword(&'a str),
    Variable(&'a str, Option<Box<Token<'a>>>),
    Wildcard(&'a str, Option<Box<Token<'a>>>),
    Boolean(bool),
    List(Vec<Token<'a>>),
    Whitespace,
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
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

impl<'a> std::fmt::Debug for Token<'a> {
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
                write!(f, "{:?}:{:?}", str, **value).unwrap();
                write!(f, "]")
            }
            Token::Wildcard(str, _) => write!(f, "_{}", str),
            Token::Atom(str) | Token::Operator(str) => write!(f, "{}", str),
            Token::Keyword(str) => write!(f, ".{}", str),
            Token::Number(n) => write!(f, ".{}", n),
            Token::Value => write!(f, "{{:value}}"),
            _ => write!(f, "{{:invalid}}"),
        }
    }
}

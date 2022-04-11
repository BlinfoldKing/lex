#[derive(Debug, Clone)]
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
            (Token::Wildcard(_, Some(b)), Token::Variable(_, Some(a)))
            | (Token::Variable(_, Some(b)), Token::Wildcard(_, Some(a))) => a == b,
            (Token::Variable(_, Some(a)), b) | (Token::Wildcard(_, Some(a)), b) => &**a == b,
            (b, Token::Variable(_, Some(a))) | (b, Token::Wildcard(_, Some(a))) => &**a == b,
            (Token::Comment, Token::Comment) | (Token::Whitespace, Token::Whitespace) => true,
            (Token::String(a), Token::String(b)) => a == b,
            (Token::Atom(a), Token::Atom(b))
            | (Token::Operator(a), Token::Operator(b))
            | (Token::Wildcard(a, None), Token::Wildcard(b, None))
            | (Token::Variable(a, None), Token::Variable(b, None))
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

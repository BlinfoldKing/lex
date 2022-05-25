use crate::grammar::token::Token;
use crate::utils::posibility::Posibility;
use std::collections::HashMap;

pub fn fill_variable(source: Token, target: Token) -> Token {
    match (source.clone(), target.clone()) {
        (x, Token::Value) => x,
        (Token::Document(doc, a), Token::Document(_, b)) => {
            Token::Document(doc, Box::new(fill_variable(*a.clone(), *b.clone())))
        }
        (Token::Definition(a, b), Token::Definition(x, y)) => {
            let a = *a.clone();
            let x = *x.clone();
            let b = *b.clone();
            let y = *y.clone();

            let def =
                Token::Definition(Box::new(fill_variable(a, x)), Box::new(fill_variable(b, y)));

            def
        }
        (Token::List(list1), Token::List(list2)) => {
            let res = list1
                .into_iter()
                .zip(list2.into_iter())
                .map(|(a, b)| fill_variable(a, b))
                .collect();
            Token::List(res)
        }
        (Token::Variable(var, Some(a)), Token::Variable(_, None)) => Token::Variable(var, Some(a)),
        (Token::Variable(x, None), Token::Variable(y, None)) => Token::Variable(
            y,
            Some(Posibility::new().with_value(Token::Variable(x, None))),
        ),
        (a, Token::Variable(var, None)) => {
            Token::Variable(var, Some(Posibility::new().with_value(a)))
        }
        _ => target,
    }
}

pub fn extract_variables(source: Token) -> HashMap<String, Posibility<Token>> {
    let mut res: HashMap<String, Posibility<Token>> = HashMap::new();
    res
}

pub fn inject_variables(source: Token, variables: HashMap<String, Token>) -> Token {
    match source {
        Token::List(list) => Token::List(
            list.into_iter()
                .map(move |token| inject_variables(token, variables.clone()))
                .collect(),
        ),
        Token::Variable(s, _) => match variables.get(&s.clone()) {
            None => Token::Variable(s, None),
            Some(Token::Variable(key, val)) => Token::Variable(key.to_owned(), val.clone()),
            Some(val) => Token::Variable(s, Some(Posibility::new().with_value(val.clone()))),
        },
        token => token,
    }
}

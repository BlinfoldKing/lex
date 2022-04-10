use crate::grammar::token::Token;

#[derive(Clone)]
pub struct BinaryOperation<'a> {
    number_fn: Option<fn(f64, f64) -> Option<Token<'a>>>,
    bool_fn: Option<fn(bool, bool) -> Option<Token<'a>>>,
}

impl<'a> BinaryOperation<'a> {
    pub fn new() -> Self {
        BinaryOperation {
            number_fn: None,
            bool_fn: None,
        }
    }

    pub fn for_number(&self, func: fn(f64, f64) -> Option<Token<'a>>) -> Self {
        let mut op = self.clone();
        op.number_fn = Some(func);

        op
    }

    pub fn exec(&self, arg: Token) -> Option<Token<'a>> {
        if let Token::List(lst) = arg {
            match &lst[..] {
                [_, Token::Variable(_, Some(x)), Token::Variable(_, Some(y))] => {
                    match (*x.clone(), *y.clone()) {
                        (Token::Number(a), Token::Number(b)) => {
                            return self.number_fn.unwrap_or(|_, _| None)(a, b)
                        }
                        (Token::Boolean(a), Token::Boolean(b)) => {
                            return self.bool_fn.unwrap_or(|_, _| None)(a, b)
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        None
    }
}

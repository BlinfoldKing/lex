use crate::ast::State;
use crate::grammar::token::Token;
use crate::handler::Handler;

#[derive(Clone)]
pub struct Definition<'a> {
    pub inp_sig: Token<'a>,
    pub out_sig: Token<'a>,
    pub func: Handler<'a>,
}

impl<'a> Definition<'a> {
    fn fill_variable(source: Token<'a>, target: Token<'a>) -> Token<'a> {
        match (source, target.clone()) {
            (x, Token::Value) => x,
            (Token::List(list1), Token::List(list2)) => {
                let res = list1
                    .into_iter()
                    .zip(list2.into_iter())
                    .map(|(a, b)| Self::fill_variable(a, b))
                    .collect();
                Token::List(res)
            }
            (Token::Variable(_, Some(a)), Token::Variable(var, None)) => {
                Token::Variable(var, Some(a))
            }
            (a, Token::Variable(var, None)) => Token::Variable(var, Some(Box::new(a))),
            _ => target,
        }
    }

    pub fn handle(&self, state: State<'a>, inp: Token<'a>) -> (State<'a>, Token<'a>) {
        let arg = Self::fill_variable(inp, self.inp_sig.clone());
        let func = &self.func;
        func(state, arg)
    }
}

impl<'a> std::fmt::Debug for Definition<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.inp_sig, self.out_sig)
    }
}

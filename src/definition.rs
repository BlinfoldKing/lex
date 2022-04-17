use crate::ast::State;
use crate::grammar::token::Token;
use crate::handler::Handler;

#[derive(Clone)]
pub struct Definition {
    pub inp_sig: Token,
    pub out_sig: Token,
    pub res_sig: Token,
    pub func: Handler,
}

impl Definition {
    pub fn fill_variable(source: Token, target: Token) -> Token {
        match (source, target.clone()) {
            (x, Token::Value) => x,
            (Token::Document(doc, a), Token::Document(_, b)) => {
                Token::Document(doc, Box::new(Self::fill_variable(*a.clone(), *b.clone())))
            }
            (Token::List(list1), Token::List(list2)) => {
                let res = list1
                    .into_iter()
                    .zip(list2.into_iter())
                    .map(|(a, b)| Self::fill_variable(a, b))
                    .collect();
                Token::List(res)
            }
            (Token::Variable(var, Some(a)), Token::Variable(_, None)) => {
                Token::Variable(var, Some(a))
            }
            (Token::Variable(x, None), Token::Variable(y, None)) => {
                Token::Variable(y, Some(Box::new(Token::Variable(x, None))))
            }
            (a, Token::Variable(var, None)) => Token::Variable(var, Some(Box::new(a))),
            _ => target,
        }
    }

    pub fn handle(&self, state: State, inp: Token) -> (State, Token) {
        let arg = Self::fill_variable(inp.clone(), self.inp_sig.clone());
        let func = &self.func;

        let (state, out) = func(state, arg);

        let res = Self::fill_variable(out.clone(), self.out_sig.clone());

        match inp {
            Token::List(list) => match &list[..] {
                [Token::Operator(op), _] => {
                    if op.clone() == "!".to_owned() || op.clone() == "?".to_owned() {
                        (state, res)
                    } else {
                        state.exec(res)
                    }
                }
                _ => state.exec(res),
            },
            _ => state.exec(res),
        }
    }
}

impl std::fmt::Debug for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} -> {}",
            self.inp_sig, self.out_sig, self.res_sig
        )
    }
}

use super::Module;
use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::state::State;
use crate::utils::operation::UnaryOperation;
use std::sync::Arc;

pub struct Fmt;

impl Module for Fmt {
    fn load(&self) -> Vec<Definition> {
        vec![Definition {
            inp_sig: Token::List(vec![Token::Identifier("println".to_owned()), Token::Value]),
            out_sig: Token::Value,
            res_sig: Token::Value,
            func: Arc::new(Box::new(Self::println)),
        }]
    }
}

impl Fmt {
    fn println(state: &mut State, arg: Token) -> Token {
        let mut op = UnaryOperation::new();
        op.for_executed_any(Box::new(|_, token| {
            println!("{}", token.clone());

            Some(token)
        }));

        match op.exec(state, arg) {
            Some(val) => val,
            _ => Token::_false(),
        }
    }
}

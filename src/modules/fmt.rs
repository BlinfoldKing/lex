use super::Module;
use crate::ast::State;
use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::utils::operation::UnaryOperation;
use std::sync::Arc;

pub struct Fmt;

impl Module for Fmt {
    fn load(&self) -> Vec<Definition> {
        vec![Definition {
            inp_sig: Token::List(vec![Token::Keyword("println".to_owned()), Token::Value]),
            out_sig: Token::Value,
            res_sig: Token::Value,
            func: Arc::new(Self::println),
        }]
    }
}

impl Fmt {
    fn println(state: State, arg: Token) -> (State, Token) {
        let op = UnaryOperation::new().for_executed_any(|s, token| {
            println!("{}", token.clone());

            Some((s, token))
        });

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }
}

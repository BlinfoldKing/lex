use super::Module;
use crate::ast::State;
use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::utils::operation::{BinaryOperation, UnaryOperation};
use std::sync::Arc;

pub struct Core;

impl<'a> Module<'a> for Core {
    fn load(&self) -> Vec<Definition<'a>> {
        vec![
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("+"),
                    Token::Variable("X", None),
                    Token::Variable("Y", None),
                ]),
                out_sig: Token::Value,
                func: Arc::new(Self::add),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("-"),
                    Token::Variable("X", None),
                    Token::Variable("Y", None),
                ]),
                out_sig: Token::Value,
                func: Arc::new(Self::min),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("*"),
                    Token::Variable("X", None),
                    Token::Variable("Y", None),
                ]),
                out_sig: Token::Value,
                func: Arc::new(Self::times),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("dec"),
                    Token::Variable("Pattern", None),
                ]),
                out_sig: Token::Value,
                func: Arc::new(Self::dec),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Operator("?"), Token::Variable("Pattern", None)]),
                out_sig: Token::Value,
                func: Arc::new(Self::query),
            },
        ]
    }
}

impl Core {
    fn add<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
        let op =
            BinaryOperation::new().for_number(|state, (a, b)| Some((state, Token::Number(a + b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn min<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
        let op =
            BinaryOperation::new().for_number(|state, (a, b)| Some((state, Token::Number(a - b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn times<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
        let op =
            BinaryOperation::new().for_number(|state, (a, b)| Some((state, Token::Number(a * b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn dec<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
        let op = UnaryOperation::new().for_any(|s, token| {
            let def = Definition {
                inp_sig: token,
                out_sig: Token::Boolean(true),
                func: Arc::new(Self::true_handler),
            };

            let r = s.add(def.clone().inp_sig, def);

            Some((r, Token::Boolean(true)))
        });

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn query<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
        let op = UnaryOperation::new().for_any(|s, token| {
            let res = s.query(token);

            Some((s, Token::List(res)))
        });

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn true_handler<'a>(state: State<'a>, _arg: Token<'a>) -> (State<'a>, Token<'a>) {
        (state, Token::Boolean(true))
    }
}

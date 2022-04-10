use super::Module;
use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::handler;
use crate::utils::operation::BinaryOperation;
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
                func: handler!(Self::add),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("-"),
                    Token::Variable("X", None),
                    Token::Variable("Y", None),
                ]),
                out_sig: Token::Value,
                func: handler!(Self::min),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("*"),
                    Token::Variable("X", None),
                    Token::Variable("Y", None),
                ]),
                out_sig: Token::Value,
                func: handler!(Self::times),
            },
        ]
    }
}

impl Core {
    fn add<'a>(arg: Token<'a>) -> Token<'a> {
        let op = BinaryOperation::new().for_number(|a, b| Some(Token::Number(a + b)));

        match op.exec(arg) {
            Some(val) => val,
            _ => Token::Boolean(false),
        }
    }

    fn min<'a>(arg: Token<'a>) -> Token<'a> {
        let op = BinaryOperation::new().for_number(|a, b| Some(Token::Number(a - b)));

        match op.exec(arg) {
            Some(val) => val,
            _ => Token::Boolean(false),
        }
    }

    fn times<'a>(arg: Token<'a>) -> Token<'a> {
        let op = BinaryOperation::new().for_number(|a, b| Some(Token::Number(a * b)));

        match op.exec(arg) {
            Some(val) => val,
            _ => Token::Boolean(false),
        }
    }
}

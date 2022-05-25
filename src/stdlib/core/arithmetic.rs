use super::Core;

use crate::grammar::token::Token;
use crate::state::State;
use crate::utils::operation::BinaryOperation;

impl Core {
    pub fn multiply(state: &mut State, arg: Token) -> Token {
        let mut op = BinaryOperation::new();
        op.for_number(Box::new(|_, (a, b)| Some(Token::Number(a * b))));

        match op.exec(state, arg) {
            Some(val) => val,
            _ => Token::_false(),
        }
    }

    pub fn div(state: &mut State, arg: Token) -> Token {
        let mut op = BinaryOperation::new();
        op.for_number(Box::new(|_, (a, b)| Some(Token::Number(a / b))));

        match op.exec(state, arg) {
            Some(val) => val,
            _ => Token::_false(),
        }
    }

    pub fn min(state: &mut State, arg: Token) -> Token {
        let mut op = BinaryOperation::new();
        op.for_number(Box::new(|_, (a, b)| Some(Token::Number(a - b))));

        match op.exec(state, arg) {
            Some(val) => val,
            _ => Token::_false(),
        }
    }

    pub fn add(state: &mut State, arg: Token) -> Token {
        let mut op = BinaryOperation::new();
        op.for_number(Box::new(|_, (a, b)| Some(Token::Number(a + b))));

        match op.exec(state, arg) {
            Some(val) => val,
            _ => Token::_false(),
        }
    }
}

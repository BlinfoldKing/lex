use super::Core;

use crate::grammar::token::Token;
use crate::state::State;
use crate::utils::operation::UnaryOperation;
use std::sync::Arc;

impl Core {
    pub fn return_value(state: &mut State, arg: Token) -> Token {
        let mut op = UnaryOperation::new();
        op.for_any(Box::new(|state, val| {
            state._return(val.clone());
            Some(val)
        }));

        match op.exec(state, arg) {
            Some(val) => val,
            _ => Token::_false(),
        }
    }

    pub fn document(state: &mut State, arg: Token) -> Token {
        if let Token::Document(_, content) = arg {
            return state.exec(*content.clone());
        }

        Token::_false()
    }

    pub fn def(state: &mut State, arg: Token) -> Token {
        if let Token::Definition(source, target) = arg {
            use crate::definition::Definition;
            let source = *source.clone();
            let target = *target.clone();

            let borrowed_target = Arc::clone(&Arc::new(target.clone()));
            state.add_to_parent(
                source.clone(),
                Definition {
                    inp_sig: source.clone(),
                    out_sig: target.clone(),
                    res_sig: source,
                    func: Arc::new(Box::new(move |state: &mut State, token: Token| -> Token {
                        let out = borrowed_target.as_ref().clone();
                        state.exec(out)
                    })),
                },
            );
            return Token::_true();
        }

        Token::_false()
    }
}

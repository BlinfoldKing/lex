pub mod arithmetic;
pub mod feature;

use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::modules::Module;
use std::sync::Arc;

pub struct Core;

impl Module for Core {
    fn load(&self) -> Vec<Definition> {
        let res = vec![
            // arithmetic
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("+".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Box::new(Self::add)),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("-".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Box::new(Self::min)),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("/".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Box::new(Self::div)),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("*".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Box::new(Self::div)),
            },
            // features
            Definition {
                inp_sig: Token::Document(Box::new(Token::Value), Box::new(Token::Value)),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Box::new(Self::document)),
            },
            Definition {
                inp_sig: Token::Definition(Box::new(Token::Value), Box::new(Token::Value)),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Box::new(Self::def)),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Keyword("return".to_owned()), Token::Value]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Box::new(Self::return_value)),
            },
        ];

        res
    }
}

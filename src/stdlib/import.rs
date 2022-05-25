use crate::definition::Definition;
use crate::grammar::document;
use crate::grammar::token::Token;
use crate::handler::Handler;
use crate::modules::Module;
use crate::state::State;
use crate::utils::operation::UnaryOperation;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Import {
    modules: Arc<HashMap<String, Box<dyn Module>>>,
}

impl Import {
    pub fn new(modules: HashMap<String, Box<dyn Module>>) -> Self {
        Self {
            modules: Arc::new(modules),
        }
    }

    pub fn import(&self) -> Box<Handler> {
        let modules = Arc::clone(&self.modules);
        let mut op = UnaryOperation::new();
        op.for_string(Box::new(move |state, str| {
            let m = &modules;

            match m.get(&str) {
                Some(module) => {
                    state.include(module.as_ref());

                    Some(Token::_true())
                }
                None => {
                    if let Ok(input) = std::fs::read_to_string(&str) {
                        let parse = document(&input);

                        let mut document = Token::_true();
                        match parse {
                            Ok((_, token)) => document = token,
                            Err(_) => (),
                        };

                        state.exec(document);
                        Some(Token::_true())
                    } else {
                        Some(Token::_false())
                    }
                }
            }
        }));

        Box::new(move |state: &mut State, token: Token| -> Token {
            match op.exec(state, token) {
                Some(val) => val,
                _ => Token::_false(),
            }
        })
    }
}

impl Module for Import {
    fn load(&self) -> Vec<Definition> {
        vec![Definition {
            inp_sig: Token::List(vec![Token::Keyword("include".to_owned()), Token::Value]),
            out_sig: Token::Value,
            res_sig: Token::Value,
            func: Arc::new(self.import()),
        }]
    }
}

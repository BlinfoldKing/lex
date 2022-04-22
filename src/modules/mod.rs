use crate::ast::Scope;
use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::utils::operation::UnaryOperation;
use std::collections::HashMap;
use std::sync::Arc;

pub mod core;
pub mod fmt;

pub trait Module {
    fn load(&self) -> Vec<Definition>;
}

pub struct Import;

impl Import {
    pub fn modules() -> HashMap<String, Box<dyn Module>> {
        let mut res: HashMap<String, Box<dyn Module>> = HashMap::new();

        res.insert("fmt".to_owned(), Box::new(fmt::Fmt));

        res
    }

    fn import(scope: Scope, arg: Token) -> (Scope, Token) {
        let op = UnaryOperation::new().for_string(|s, str| {
            // TODO: can be quite costly to do this too often
            let m = Self::modules();

            match m.get(&str) {
                Some(module) => {
                    let defs = module.load();
                    let mut ns = s.clone();
                    for def in defs {
                        let s = ns.add(def.clone().inp_sig, def);
                        ns = s;
                    }
                    Some((ns, Token::Boolean(true)))
                }
                None => Some((s, Token::Boolean(false))),
            }
        });

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }
}

impl Module for Import {
    fn load(&self) -> Vec<Definition> {
        vec![Definition {
            inp_sig: Token::List(vec![Token::Keyword("include".to_owned()), Token::Value]),
            out_sig: Token::Value,
            res_sig: Token::Value,
            func: Arc::new(Self::import),
        }]
    }
}

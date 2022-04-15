use super::Module;
use crate::ast::State;
use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::utils::map::Map;
use crate::utils::operation::{BinaryOperation, UnaryOperation};
use std::sync::Arc;

pub struct Core;

impl Module for Core {
    fn load(&self) -> Vec<Definition> {
        vec![
            Definition {
                inp_sig: Token::Document(
                    Box::new(Token::Variable("Name".to_owned(), None)),
                    Box::new(Token::Variable("List".to_owned(), None)),
                ),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::document),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("+".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::add),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("-".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::min),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("*".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::times),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("/".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::div_float),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("div".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::div_int),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("mod".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::mod_),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("and".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::List(vec![
                    Token::Keyword("and".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                func: Arc::new(Self::and),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("or".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::List(vec![
                    Token::Keyword("or".to_owned()),
                    Token::Variable("X".to_owned(), None),
                    Token::Variable("Y".to_owned(), None),
                ]),
                func: Arc::new(Self::or),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("do".to_owned()),
                    Token::Variable("List".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::List(vec![
                    Token::Keyword("do".to_owned()),
                    Token::Variable("List".to_owned(), None),
                ]),
                func: Arc::new(Self::do_),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("dec".to_owned()),
                    Token::Variable("Pattern".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::dec),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("def".to_owned()),
                    Token::Variable("Pattern".to_owned(), None),
                    Token::Variable("Condition".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::def),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("?".to_owned()),
                    Token::Variable("Pattern".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::query),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("!".to_owned()),
                    Token::Variable("Pattern".to_owned(), None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::find),
            },
        ]
    }
}

impl Core {
    fn add(state: State, arg: Token) -> (State, Token) {
        let op =
            BinaryOperation::new().for_number(|state, (a, b)| Some((state, Token::Number(a + b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn min(state: State, arg: Token) -> (State, Token) {
        let op =
            BinaryOperation::new().for_number(|state, (a, b)| Some((state, Token::Number(a - b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn times(state: State, arg: Token) -> (State, Token) {
        let op =
            BinaryOperation::new().for_number(|state, (a, b)| Some((state, Token::Number(a * b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn dec(state: State, arg: Token) -> (State, Token) {
        let op = UnaryOperation::new().for_any(|s, token| {
            let def = Definition {
                inp_sig: token.clone(),
                out_sig: Token::Boolean(true),
                res_sig: token,
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

    fn def(state: State, arg: Token) -> (State, Token) {
        let op = BinaryOperation::new().for_any(|s, (pattern, condition)| {
            let def = Definition {
                inp_sig: pattern.clone(),
                out_sig: condition.clone(),
                res_sig: pattern,
                func: Arc::new(Self::def_handler),
            };

            let r = s.add(def.clone().inp_sig, def);

            Some((r, Token::Boolean(true)))
        });

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn def_handler(state: State, arg: Token) -> (State, Token) {
        let def = state.find(arg.clone()).unwrap();
        let res = Self::construct_input(arg.clone(), def.clone().out_sig);
        (state, res)
    }

    fn construct_input(source: Token, target: Token) -> Token {
        let vars = Self::get_variables(source.clone());
        let res = Self::inject_variables(target.clone(), vars);
        res
    }

    fn inject_variables(source: Token, variables: Map<String, Token>) -> Token {
        match source {
            Token::List(list) => Token::List(
                list.into_iter()
                    .map(move |token| Self::inject_variables(token, variables.clone()))
                    .collect(),
            ),
            Token::Variable(s, _) => match variables.get(s.clone()) {
                None => Token::Variable(s, None),
                Some(Token::Variable(key, val)) => Token::Variable(key.to_owned(), val.clone()),
                Some(val) => Token::Variable(s, Some(Box::new(val.clone()))),
            },
            token => token,
        }
    }

    fn get_variables(source: Token) -> Map<String, Token> {
        let mut res: Map<String, Token> = Map::new();
        match source.clone() {
            Token::List(list) => {
                let vars: Vec<Map<String, Token>> = list
                    .into_iter()
                    .map(|token| Self::get_variables(token))
                    .collect();
                for var in vars {
                    res.extend(var);
                }
            }
            Token::Variable(x, None) => res.insert(x, source),
            Token::Variable(var, Some(value)) => match *value.clone() {
                Token::Variable(_, _) => res.insert(var, *value),
                x => res.insert(var, x),
            },
            _ => (),
        };

        res
    }

    fn query(state: State, arg: Token) -> (State, Token) {
        let op = UnaryOperation::new().for_any(|s, token| {
            let res = s.query(token.clone());
            if res.len() < 1 {
                return Some((s, token));
            };

            let mut list: Vec<Token> = vec![];
            let mut s = s.clone();
            for def in res {
                let inp = Definition::fill_variable(token.clone(), def.inp_sig.clone());
                let vars = Self::get_variables(inp);
                let cond_sig = Self::inject_variables(def.out_sig.clone(), vars.clone());

                let (r, found) = s.exec(Token::List(vec![
                    Token::Operator("?".to_owned()),
                    cond_sig.clone(),
                ]));
                let (r, cond) = r.exec(found.clone());

                match cond {
                    Token::Boolean(true) => {
                        let child = Definition::fill_variable(found.clone(), cond_sig.clone());
                        let value_vars = Self::get_variables(child);
                        let item = Self::inject_variables(
                            Self::inject_variables(def.res_sig, vars),
                            value_vars,
                        );

                        list.push(item);
                    }
                    Token::List(founds) => {
                        for found in founds {
                            let child = Definition::fill_variable(found.clone(), cond_sig.clone());
                            let value_vars = Self::get_variables(child);
                            let item = Self::inject_variables(
                                Self::inject_variables(def.res_sig.clone(), vars.clone()),
                                value_vars,
                            );

                            list.push(item);
                        }
                    }
                    _ => (),
                }
                s = r;
            }

            Some((s, Token::List(list)))
        });

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn find(state: State, arg: Token) -> (State, Token) {
        let op = UnaryOperation::new().for_any(|s, token| {
            let def = s.find(token.clone());
            match def {
                None => return Some((s, token)),
                Some(_) => (),
            }
            let def = def.unwrap();
            let inp = Definition::fill_variable(token.clone(), def.inp_sig.clone());
            let vars = Self::get_variables(inp);
            let cond_sig = Self::inject_variables(def.out_sig.clone(), vars.clone());

            let (s, found) = s.exec(Token::List(vec![
                Token::Operator("!".to_owned()),
                cond_sig.clone(),
            ]));
            let (s, cond) = s.exec(found.clone());

            match cond {
                Token::Boolean(true) => {
                    let child = Definition::fill_variable(found.clone(), cond_sig.clone());
                    let value_vars = Self::get_variables(child);
                    let res = Self::inject_variables(
                        Self::inject_variables(def.res_sig, vars),
                        value_vars,
                    );

                    Some((s, res))
                }
                token => Some((s, token)),
            }
        });

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn div_int(state: State, arg: Token) -> (State, Token) {
        let op = BinaryOperation::new()
            .for_number(|state, (a, b)| Some((state, Token::Number((a as i64 / b as i64) as f64))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn div_float(state: State, arg: Token) -> (State, Token) {
        let op =
            BinaryOperation::new().for_number(|state, (a, b)| Some((state, Token::Number(a / b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn mod_(state: State, arg: Token) -> (State, Token) {
        let op =
            BinaryOperation::new().for_number(|state, (a, b)| Some((state, Token::Number(a % b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn and(state: State, arg: Token) -> (State, Token) {
        let op = BinaryOperation::new()
            .for_boolean(|state, (a, b)| Some((state, Token::Boolean(a && b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn or(state: State, arg: Token) -> (State, Token) {
        let op = BinaryOperation::new()
            .for_boolean(|state, (a, b)| Some((state, Token::Boolean(a || b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn do_(state: State, arg: Token) -> (State, Token) {
        let op = UnaryOperation::new()
            .for_list(|state, list| {
                let mut state = state.clone();
                let mut items: Vec<Token> = vec![];
                for item in list {
                    let (s, val) = state.exec(item.clone());
                    state = s;
                    items.push(val);
                }

                let ret = items.into_iter().fold(true, |res, val| match val {
                    Token::Boolean(b) => res && b,
                    _ => false,
                });

                Some((state, Token::Boolean(ret)))
            })
            .for_boolean(|state, b| Some((state, Token::Boolean(b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn document(state: State, arg: Token) -> (State, Token) {
        if let Token::Document(_, token) = arg {
            let res = state.exec(Token::List(vec![
                Token::Keyword("do".to_owned()),
                *token.clone(),
            ]));
            return res;
        }

        (state, Token::Boolean(false))
    }

    fn true_handler(state: State, _: Token) -> (State, Token) {
        (state, Token::Boolean(true))
    }
}

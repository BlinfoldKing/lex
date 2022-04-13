use super::Module;
use crate::ast::State;
use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::utils::map::Map;
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
                res_sig: Token::Value,
                func: Arc::new(Self::add),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("-"),
                    Token::Variable("X", None),
                    Token::Variable("Y", None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::min),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("*"),
                    Token::Variable("X", None),
                    Token::Variable("Y", None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::times),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("dec"),
                    Token::Variable("Pattern", None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::dec),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("def"),
                    Token::Variable("Pattern", None),
                    Token::Variable("Condition", None),
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::def),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Operator("?"), Token::Variable("Pattern", None)]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::query),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Operator("!"), Token::Variable("Pattern", None)]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::find),
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

    fn def<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
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

    fn def_handler<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
        let def = state.find(arg.clone()).unwrap();
        let res = Self::construct_input(arg.clone(), def.clone().out_sig);
        (state, res)
    }

    fn construct_input<'a>(source: Token<'a>, target: Token<'a>) -> Token<'a> {
        let vars = Self::get_variables(source.clone());
        let res = Self::inject_variables(target.clone(), vars);
        res
    }

    fn inject_variables<'a>(source: Token<'a>, variables: Map<&str, Token<'a>>) -> Token<'a> {
        match source {
            Token::List(list) => Token::List(
                list.into_iter()
                    .map(move |token| Self::inject_variables(token, variables.clone()))
                    .collect(),
            ),
            Token::Variable(s, _) => match variables.get(s) {
                None => Token::Variable(s, None),
                Some(Token::Variable(key, val)) => Token::Variable(key, val.clone()),
                Some(val) => Token::Variable(s, Some(Box::new(val.clone()))),
            },
            token => token,
        }
    }

    fn get_variables<'a>(source: Token<'a>) -> Map<&str, Token<'a>> {
        let mut res: Map<&str, Token<'a>> = Map::new();
        match source.clone() {
            Token::List(list) => {
                let vars: Vec<Map<&str, Token<'a>>> = list
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

    fn query<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
        let op = UnaryOperation::new().for_any(|s, token| {
            let res = s.query(token.clone());
            if res.len() < 1 {
                return Some((s, token));
            };

            let mut list: Vec<Token<'a>> = vec![];
            let mut s = s.clone();
            for def in res {
                let inp = Definition::fill_variable(token.clone(), def.inp_sig.clone());
                let vars = Self::get_variables(inp);
                let cond_sig = Self::inject_variables(def.out_sig.clone(), vars.clone());

                let (r, found) = s.exec(Token::List(vec![Token::Operator("?"), cond_sig.clone()]));
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

    fn find<'a>(state: State<'a>, arg: Token<'a>) -> (State<'a>, Token<'a>) {
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

            let (s, found) = s.exec(Token::List(vec![Token::Operator("!"), cond_sig.clone()]));
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

    fn true_handler<'a>(state: State<'a>, _: Token<'a>) -> (State<'a>, Token<'a>) {
        (state, Token::Boolean(true))
    }
}

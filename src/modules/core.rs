use super::Module;
use crate::ast::State;
use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::utils::operation::{BinaryOperation, UnaryOperation};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Core;

impl Module for Core {
    fn load(&self) -> Vec<Definition> {
        vec![
            Definition {
                inp_sig: Token::List(vec![Token::Keyword("debug".to_owned())]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::debug_state),
            },
            Definition {
                inp_sig: Token::Document(Box::new(Token::Value), Box::new(Token::Value)),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::document),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("+".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::add),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("=".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::eq),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("-".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::min),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Operator("*".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::times),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Operator("/".to_owned()), Token::Value]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::div_float),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("div".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::div_int),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("mod".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::mod_),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("and".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::List(vec![
                    Token::Keyword("and".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                func: Arc::new(Self::and),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("or".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::List(vec![
                    Token::Keyword("or".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                func: Arc::new(Self::or),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Keyword("do".to_owned()), Token::Value]),
                out_sig: Token::Value,
                res_sig: Token::List(vec![Token::Keyword("do".to_owned()), Token::Value]),
                func: Arc::new(Self::do_),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Keyword("dec".to_owned()), Token::Value]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::dec),
            },
            Definition {
                inp_sig: Token::List(vec![
                    Token::Keyword("def".to_owned()),
                    Token::Value,
                    Token::Value,
                ]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::def),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Operator("?".to_owned()), Token::Value]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::query),
            },
            Definition {
                inp_sig: Token::List(vec![Token::Operator("!".to_owned()), Token::Value]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::find),
            },
        ]
    }
}

impl Core {
    fn eq(state: State, arg: Token) -> (State, Token) {
        let op =
            BinaryOperation::new().for_any(|state, (a, b)| Some((state, Token::Boolean(a == b))));

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

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
                res_sig: token.clone(),
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
                res_sig: pattern.clone(),
                func: Arc::new(Self::def_handler),
            };

            // println!("\nbefore:\n {:?}\n", s);

            let r = s.add(def.clone().inp_sig, def);

            // println!("pattern: {:?}", pattern.clone());
            // println!("condition: {:?}", condition.clone());

            // println!("\nstate:\n {:?}\n", r);
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

    fn inject_variables(source: Token, variables: HashMap<String, Token>) -> Token {
        match source {
            Token::List(list) => Token::List(
                list.into_iter()
                    .map(move |token| Self::inject_variables(token, variables.clone()))
                    .collect(),
            ),
            Token::Variable(s, _) => match variables.get(&s.clone()) {
                None => Token::Variable(s, None),
                Some(Token::Variable(key, val)) => Token::Variable(key.to_owned(), val.clone()),
                Some(val) => Token::Variable(s, Some(Box::new(val.clone()))),
            },
            token => token,
        }
    }

    fn get_variables(source: Token) -> HashMap<String, Token> {
        let mut res: HashMap<String, Token> = HashMap::new();
        match source.clone() {
            Token::List(list) => {
                let vars: Vec<HashMap<String, Token>> = list
                    .into_iter()
                    .map(|token| Self::get_variables(token))
                    .collect();
                for var in vars {
                    res.extend(var);
                }
            }
            Token::Variable(x, None) => {
                let _ = res.insert(x, source);
            }
            Token::Variable(var, Some(value)) => {
                match *value.clone() {
                    Token::Variable(_, _) => res.insert(var, *value),
                    x => res.insert(var, x),
                };
            }
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
            let args = Self::get_variables(Definition::fill_variable(
                token.clone(),
                def.inp_sig.clone(),
            ));

            match def.out_sig.clone() {
                Token::Value => {
                    let inp = Definition::fill_variable(token.clone(), def.clone().inp_sig);
                    let params = Self::extract_value(inp.clone(), def.inp_sig);

                    let mut s = s.clone();
                    let mut new_params: Vec<Token> = vec![];
                    for param in params.clone() {
                        let (r, token) =
                            s.exec(Token::List(vec![Token::Operator("!".to_owned()), param]));
                        s = r;
                        new_params.push(token);
                    }

                    let (res, _) = Self::inject_value(def.res_sig, new_params);
                    return Some((s, res));
                }
                out => {
                    let inp = Self::inject_variables(out.clone(), args.clone());

                    let (r, t) = s.exec(Token::List(vec![
                        Token::Operator("!".to_owned()),
                        inp.clone(),
                    ]));

                    let t = Definition::fill_variable(t, inp);
                    let vars = Self::get_variables(t);

                    let res = Self::inject_variables(def.res_sig, args);
                    let res = Self::inject_variables(res, vars);

                    return Some((r, res));
                }
            }
        });

        match op.exec(state.clone(), arg) {
            Some(val) => val,
            _ => (state, Token::Boolean(false)),
        }
    }

    fn extract_value(source: Token, target: Token) -> Vec<Token> {
        match (source, target) {
            (Token::List(l1), Token::List(l2)) => {
                let res: Vec<Token> = l1
                    .into_iter()
                    .zip(l2.into_iter())
                    .map(|(a, b)| Self::extract_value(a, b))
                    .flatten()
                    .collect();
                res
            }
            (a, Token::Value) => vec![a],
            _ => vec![],
        }
    }

    fn inject_value(source: Token, values: Vec<Token>) -> (Token, Vec<Token>) {
        match source {
            Token::List(list) => {
                let mut val = values.clone();
                let mut tokens: Vec<Token> = vec![];
                for elem in list {
                    let (t, v) = Self::inject_value(elem, val);
                    val = v;
                    tokens.push(t);
                }

                (Token::List(tokens), val)
            }
            Token::Value => {
                if values.len() == 0 {
                    return (source, values);
                }

                let head = values.first().unwrap();
                let tail = values[1..].to_vec();

                (head.clone(), tail)
            }
            _ => (source, values),
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

    fn debug_state(state: State, _: Token) -> (State, Token) {
        println!("{:?}", state);

        (state, Token::Boolean(true))
    }

    fn true_handler(state: State, _: Token) -> (State, Token) {
        (state, Token::Boolean(true))
    }
}

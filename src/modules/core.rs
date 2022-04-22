use super::Module;
use crate::ast::Scope;
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
                func: Arc::new(Self::debug_scope),
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
            Definition {
                inp_sig: Token::List(vec![Token::Keyword("return".to_owned()), Token::Value]),
                out_sig: Token::Value,
                res_sig: Token::Value,
                func: Arc::new(Self::ret),
            },
        ]
    }
}

impl Core {
    fn eq(scope: Scope, arg: Token) -> (Scope, Token) {
        let op =
            BinaryOperation::new().for_any(|scope, (a, b)| Some((scope, Token::Boolean(a == b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn add(scope: Scope, arg: Token) -> (Scope, Token) {
        let op =
            BinaryOperation::new().for_number(|scope, (a, b)| Some((scope, Token::Number(a + b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn min(scope: Scope, arg: Token) -> (Scope, Token) {
        let op =
            BinaryOperation::new().for_number(|scope, (a, b)| Some((scope, Token::Number(a - b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn times(scope: Scope, arg: Token) -> (Scope, Token) {
        let op =
            BinaryOperation::new().for_number(|scope, (a, b)| Some((scope, Token::Number(a * b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn dec(scope: Scope, arg: Token) -> (Scope, Token) {
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

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn def(scope: Scope, arg: Token) -> (Scope, Token) {
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

            // println!("\nscope:\n {:?}\n", r);
            Some((r, Token::Boolean(true)))
        });

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn def_handler(scope: Scope, arg: Token) -> (Scope, Token) {
        let def = scope.find(arg.clone()).unwrap();
        let res = Self::construct_input(arg.clone(), def.clone().out_sig);
        (scope, res)
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

    fn query(scope: Scope, arg: Token) -> (Scope, Token) {
        let op = UnaryOperation::new().for_any(|s, token| {
            let res = s.query(token.clone());

            let mut list: Vec<Token> = vec![];
            let mut s = s.clone();
            for def in res {
                let args = Self::get_variables(Definition::fill_variable(
                    token.clone(),
                    def.inp_sig.clone(),
                ));

                match def.out_sig.clone() {
                    Token::Value => {}
                    out => {
                        let inp = Self::inject_variables(out.clone(), args.clone());

                        let (r, t) = s.exec(Token::List(vec![
                            Token::Operator("?".to_owned()),
                            inp.clone(),
                        ]));

                        let t = Definition::fill_variable(t, inp);
                        let vars = Self::get_variables(t);

                        let res = Self::inject_variables(def.res_sig, args);
                        let res = Self::inject_variables(res, vars);

                        list.push(res);
                        s = r;
                    }
                };
            }

            Some((s, Token::List(list)))
        });

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn find(scope: Scope, arg: Token) -> (Scope, Token) {
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

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
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

    fn div_int(scope: Scope, arg: Token) -> (Scope, Token) {
        let op = BinaryOperation::new()
            .for_number(|scope, (a, b)| Some((scope, Token::Number((a as i64 / b as i64) as f64))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn div_float(scope: Scope, arg: Token) -> (Scope, Token) {
        let op =
            BinaryOperation::new().for_number(|scope, (a, b)| Some((scope, Token::Number(a / b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn mod_(scope: Scope, arg: Token) -> (Scope, Token) {
        let op =
            BinaryOperation::new().for_number(|scope, (a, b)| Some((scope, Token::Number(a % b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn and(scope: Scope, arg: Token) -> (Scope, Token) {
        let op = BinaryOperation::new()
            .for_boolean(|scope, (a, b)| Some((scope, Token::Boolean(a && b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn or(scope: Scope, arg: Token) -> (Scope, Token) {
        let op = BinaryOperation::new()
            .for_boolean(|scope, (a, b)| Some((scope, Token::Boolean(a || b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn do_(scope: Scope, arg: Token) -> (Scope, Token) {
        let op = UnaryOperation::new()
            .for_list(|scope, list| {
                let mut scope = scope.clone();
                let mut items: Vec<Token> = vec![];
                for item in list {
                    let (s, val) = scope.exec(item.clone());
                    scope = s;
                    items.push(val);
                }

                let ret = items.into_iter().fold(true, |res, val| match val {
                    Token::Boolean(b) => res && b,
                    _ => false,
                });

                Some((scope, Token::Boolean(ret)))
            })
            .for_boolean(|scope, b| Some((scope, Token::Boolean(b))));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }

    fn document(scope: Scope, arg: Token) -> (Scope, Token) {
        if let Token::Document(_, token) = arg {
            let res = scope.exec(Token::List(vec![
                Token::Keyword("do".to_owned()),
                *token.clone(),
            ]));
            return res;
        }

        (scope, Token::Boolean(false))
    }

    fn debug_scope(scope: Scope, _: Token) -> (Scope, Token) {
        println!("{:?}", scope);

        (scope, Token::Boolean(true))
    }

    fn true_handler(scope: Scope, _: Token) -> (Scope, Token) {
        (scope, Token::Boolean(true))
    }

    fn ret(scope: Scope, arg: Token) -> (Scope, Token) {
        let op = UnaryOperation::new()
            .for_executed_any(|scope, val| Some((scope.ret(val.clone()), val)));

        match op.exec(scope.clone(), arg) {
            Some(val) => val,
            _ => (scope, Token::Boolean(false)),
        }
    }
}

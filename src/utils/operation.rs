use crate::grammar::token::Token;
use crate::state::State;

type BinaryOpCallback<T> = Box<dyn Fn(&mut State, (T, T)) -> Option<Token>>;

pub struct BinaryOperation {
    number_fn: Option<BinaryOpCallback<f64>>,
    boolean_fn: Option<BinaryOpCallback<bool>>,
    atom_fn: Option<BinaryOpCallback<String>>,
    string_fn: Option<BinaryOpCallback<String>>,
    list_fn: Option<BinaryOpCallback<Vec<Token>>>,
    any_fn: Option<BinaryOpCallback<Token>>,
}

impl BinaryOperation {
    pub fn new() -> Self {
        BinaryOperation {
            number_fn: None,
            boolean_fn: None,
            atom_fn: None,
            string_fn: None,
            list_fn: None,
            any_fn: None,
        }
    }

    pub fn for_number(&mut self, func: BinaryOpCallback<f64>) -> &mut Self {
        self.number_fn = Some(func);

        self
    }

    pub fn for_boolean(&mut self, func: BinaryOpCallback<bool>) -> &mut Self {
        self.boolean_fn = Some(func);

        self
    }

    pub fn for_atom(&mut self, func: BinaryOpCallback<String>) -> &mut Self {
        self.atom_fn = Some(func);

        self
    }

    pub fn for_string(&mut self, func: BinaryOpCallback<String>) -> &mut Self {
        self.string_fn = Some(func);

        self
    }

    pub fn for_list(&mut self, func: BinaryOpCallback<Vec<Token>>) -> &mut Self {
        self.list_fn = Some(func);

        self
    }

    pub fn for_any(&mut self, func: BinaryOpCallback<Token>) -> &mut Self {
        self.any_fn = Some(func);

        self
    }

    pub fn exec(&mut self, state: &mut State, arg: Token) -> Option<Token> {
        if let Token::List(lst) = arg {
            match &lst[..] {
                // [op, Token::Variable(_, Some(a)), Token::Variable(_, Some(b))] => {
                //     return self.exec(state, Token::List(vec![op.clone(), *a.clone(), *b.clone()]))
                // }
                [_, x, y] => {
                    if let Some(func) = &self.any_fn {
                        return func(state, (x.clone(), y.clone()));
                    }

                    let x = state.exec(x.clone());
                    let y = state.exec(y.clone());

                    match (x, y) {
                        (Token::Number(a), Token::Number(b)) => {
                            if let Some(func) = &self.number_fn {
                                return func(state, (a, b));
                            }
                        }
                        (Token::Boolean(a), Token::Boolean(b)) => {
                            if let Some(func) = &self.boolean_fn {
                                return func(state, (a, b));
                            }
                        }
                        (Token::Atom(a), Token::Atom(b)) => {
                            if let Some(func) = &self.atom_fn {
                                return func(state, (a, b));
                            }
                        }
                        (Token::String(a), Token::String(b)) => {
                            if let Some(func) = &self.string_fn {
                                return func(state, (a, b));
                            }
                        }
                        (Token::List(a), Token::List(b)) => {
                            if let Some(func) = &self.list_fn {
                                return func(state, (a, b));
                            }
                        }
                        (a, b) => {
                            if let Some(func) = &self.any_fn {
                                return func(state, (a, b));
                            }
                        }
                    };

                    return None;
                }
                _ => (),
            }
        }
        None
    }
}

type UnaryOpCallback<T> = Box<dyn Fn(&mut State, T) -> Option<Token>>;

pub struct UnaryOperation {
    number_fn: Option<UnaryOpCallback<f64>>,
    boolean_fn: Option<UnaryOpCallback<bool>>,
    atom_fn: Option<UnaryOpCallback<String>>,
    string_fn: Option<UnaryOpCallback<String>>,
    list_fn: Option<UnaryOpCallback<Vec<Token>>>,
    any_fn: Option<UnaryOpCallback<Token>>,
    executed_any_fn: Option<UnaryOpCallback<Token>>,
}

impl UnaryOperation {
    pub fn new() -> Self {
        UnaryOperation {
            number_fn: None,
            boolean_fn: None,
            atom_fn: None,
            string_fn: None,
            list_fn: None,
            any_fn: None,
            executed_any_fn: None,
        }
    }

    pub fn for_number(&mut self, func: UnaryOpCallback<f64>) -> &mut Self {
        self.number_fn = Some(func);

        self
    }

    pub fn for_boolean(&mut self, func: UnaryOpCallback<bool>) -> &Self {
        self.boolean_fn = Some(func);

        self
    }

    pub fn for_atom(&mut self, func: UnaryOpCallback<String>) -> &Self {
        self.atom_fn = Some(func);

        self
    }

    pub fn for_string(&mut self, func: UnaryOpCallback<String>) -> &Self {
        self.string_fn = Some(func);

        self
    }

    pub fn for_list(&mut self, func: UnaryOpCallback<Vec<Token>>) -> &Self {
        self.list_fn = Some(func);

        self
    }

    pub fn for_any(&mut self, func: UnaryOpCallback<Token>) -> &Self {
        self.any_fn = Some(func);

        self
    }

    pub fn for_executed_any(&mut self, func: UnaryOpCallback<Token>) -> &Self {
        self.executed_any_fn = Some(func);

        self
    }

    pub fn exec(&self, state: &mut State, arg: Token) -> Option<Token> {
        if let Token::List(lst) = arg {
            match &lst[..] {
                // [op, Token::Variable(_, Some(x))] => {
                //     x.values().iter().map
                //     return self.exec(state, Token::List(vec![op.clone(), x.clone()]))
                // }
                [_, x] => {
                    let inp = state.exec(x.clone());

                    if let Some(func) = &self.any_fn {
                        return func(state, x.clone());
                    }

                    if let Some(func) = &self.executed_any_fn {
                        return func(state, inp);
                    }

                    match inp {
                        Token::Number(a) => {
                            if let Some(func) = &self.number_fn {
                                return func(state, a);
                            }
                        }
                        Token::Boolean(a) => {
                            if let Some(func) = &self.boolean_fn {
                                return func(state, a);
                            }
                        }
                        Token::Atom(a) => {
                            if let Some(func) = &self.atom_fn {
                                return func(state, a);
                            }
                        }
                        Token::String(a) => {
                            if let Some(func) = &self.string_fn {
                                return func(state, a);
                            }
                        }
                        Token::List(a) => {
                            if let Some(func) = &self.list_fn {
                                return func(state, a);
                            }
                        }
                        a => {
                            if let Some(func) = &self.any_fn {
                                return func(state, a);
                            }
                        }
                    };

                    return None;
                }
                _ => (),
            }
        }
        None
    }
}

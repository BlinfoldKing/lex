use crate::ast::State;
use crate::grammar::token::Token;

type BinaryOpCallback<T> = fn(State, (T, T)) -> Option<(State, Token)>;

#[derive(Clone)]
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

    pub fn for_number(&self, func: BinaryOpCallback<f64>) -> Self {
        let mut op = self.clone();
        op.number_fn = Some(func);

        op
    }

    pub fn for_boolean(&self, func: BinaryOpCallback<bool>) -> Self {
        let mut op = self.clone();
        op.boolean_fn = Some(func);

        op
    }

    pub fn for_atom(&self, func: BinaryOpCallback<String>) -> Self {
        let mut op = self.clone();
        op.atom_fn = Some(func);

        op
    }

    pub fn for_string(&self, func: BinaryOpCallback<String>) -> Self {
        let mut op = self.clone();
        op.string_fn = Some(func);

        op
    }

    pub fn for_list(&self, func: BinaryOpCallback<Vec<Token>>) -> Self {
        let mut op = self.clone();
        op.list_fn = Some(func);

        op
    }

    pub fn for_any(&self, func: BinaryOpCallback<Token>) -> Self {
        let mut op = self.clone();
        op.any_fn = Some(func);

        op
    }

    pub fn exec(&self, state: State, arg: Token) -> Option<(State, Token)> {
        if let Token::List(lst) = arg {
            match &lst[..] {
                [op, Token::Variable(_, Some(a)), Token::Variable(_, Some(b))] => {
                    return self
                        .clone()
                        .exec(state, Token::List(vec![op.clone(), *a.clone(), *b.clone()]))
                }
                [_, x, y] => {
                    if self.any_fn != None {
                        return self.any_fn.unwrap()(state, (x.clone(), y.clone()));
                    }

                    let (state, x) = state.exec(x.clone());
                    let (state, y) = state.exec(y.clone());

                    match (x, y) {
                        (Token::Number(a), Token::Number(b)) => {
                            return self.number_fn.unwrap_or(|_, _| None)(state, (a, b))
                        }
                        (Token::Boolean(a), Token::Boolean(b)) => {
                            return self.boolean_fn.unwrap_or(|_, _| None)(state, (a, b))
                        }
                        (Token::Atom(a), Token::Atom(b)) => {
                            return self.atom_fn.unwrap_or(|_, _| None)(state, (a, b))
                        }
                        (Token::String(a), Token::String(b)) => {
                            return self.string_fn.unwrap_or(|_, _| None)(state, (a, b))
                        }
                        (Token::List(a), Token::List(b)) => {
                            return self.list_fn.unwrap_or(|_, _| None)(state, (a, b))
                        }
                        (a, b) => return self.any_fn.unwrap_or(|_, _| None)(state, (a, b)),
                    }
                }
                _ => (),
            }
        }
        None
    }
}

type UnaryOpCallback<T> = fn(State, T) -> Option<(State, Token)>;

#[derive(Clone)]
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

    pub fn for_number(&self, func: UnaryOpCallback<f64>) -> Self {
        let mut op = self.clone();
        op.number_fn = Some(func);

        op
    }

    pub fn for_boolean(&self, func: UnaryOpCallback<bool>) -> Self {
        let mut op = self.clone();
        op.boolean_fn = Some(func);

        op
    }

    pub fn for_atom(&self, func: UnaryOpCallback<String>) -> Self {
        let mut op = self.clone();
        op.atom_fn = Some(func);

        op
    }

    pub fn for_string(&self, func: UnaryOpCallback<String>) -> Self {
        let mut op = self.clone();
        op.string_fn = Some(func);

        op
    }

    pub fn for_list(&self, func: UnaryOpCallback<Vec<Token>>) -> Self {
        let mut op = self.clone();
        op.list_fn = Some(func);

        op
    }

    pub fn for_any(&self, func: UnaryOpCallback<Token>) -> Self {
        let mut op = self.clone();
        op.any_fn = Some(func);

        op
    }

    pub fn for_executed_any(&self, func: UnaryOpCallback<Token>) -> Self {
        let mut op = self.clone();
        op.executed_any_fn = Some(func);

        op
    }

    pub fn exec(&self, state: State, arg: Token) -> Option<(State, Token)> {
        if let Token::List(lst) = arg {
            match &lst[..] {
                [op, Token::Variable(_, Some(x))] => {
                    return self
                        .clone()
                        .exec(state, Token::List(vec![op.clone(), *x.clone()]))
                }
                [_, x] => {
                    if self.any_fn != None {
                        return self.any_fn.unwrap()(state, x.clone());
                    }

                    let (state, inp) = state.exec(x.clone());

                    if self.executed_any_fn != None {
                        return self.executed_any_fn.unwrap()(state, inp);
                    }

                    match inp {
                        Token::Number(a) => return self.number_fn.unwrap_or(|_, _| None)(state, a),
                        Token::Boolean(a) => {
                            return self.boolean_fn.unwrap_or(|_, _| None)(state, a)
                        }
                        Token::Atom(a) => return self.atom_fn.unwrap_or(|_, _| None)(state, a),
                        Token::String(a) => return self.string_fn.unwrap_or(|_, _| None)(state, a),
                        Token::List(a) => return self.list_fn.unwrap_or(|_, _| None)(state, a),
                        a => return self.any_fn.unwrap_or(|_, _| None)(state, a),
                    }
                }
                _ => (),
            }
        }
        None
    }
}

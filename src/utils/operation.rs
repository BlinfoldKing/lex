use crate::ast::State;
use crate::grammar::token::Token;

type BinaryOpCallback<'a, T> = fn(State<'a>, (T, T)) -> Option<(State<'a>, Token<'a>)>;

#[derive(Clone)]
pub struct BinaryOperation<'a> {
    number_fn: Option<BinaryOpCallback<'a, f64>>,
    boolean_fn: Option<BinaryOpCallback<'a, bool>>,
    atom_fn: Option<BinaryOpCallback<'a, &'a str>>,
    string_fn: Option<BinaryOpCallback<'a, String>>,
    list_fn: Option<BinaryOpCallback<'a, Vec<Token<'a>>>>,
    any_fn: Option<BinaryOpCallback<'a, Token<'a>>>,
}

impl<'a> BinaryOperation<'a> {
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

    pub fn for_number(&self, func: BinaryOpCallback<'a, f64>) -> Self {
        let mut op = self.clone();
        op.number_fn = Some(func);

        op
    }

    pub fn for_boolean(&self, func: BinaryOpCallback<'a, bool>) -> Self {
        let mut op = self.clone();
        op.boolean_fn = Some(func);

        op
    }

    pub fn for_atom(&self, func: BinaryOpCallback<'a, &'a str>) -> Self {
        let mut op = self.clone();
        op.atom_fn = Some(func);

        op
    }

    pub fn for_string(&self, func: BinaryOpCallback<'a, String>) -> Self {
        let mut op = self.clone();
        op.string_fn = Some(func);

        op
    }

    pub fn for_list(&self, func: BinaryOpCallback<'a, Vec<Token<'a>>>) -> Self {
        let mut op = self.clone();
        op.list_fn = Some(func);

        op
    }

    pub fn for_any(&self, func: BinaryOpCallback<'a, Token<'a>>) -> Self {
        let mut op = self.clone();
        op.any_fn = Some(func);

        op
    }

    pub fn exec(&self, state: State<'a>, arg: Token<'a>) -> Option<(State<'a>, Token<'a>)> {
        if let Token::List(lst) = arg {
            match &lst[..] {
                [_, Token::Variable(_, Some(x)), Token::Variable(_, Some(y))] => {
                    if self.any_fn != None {
                        return self.any_fn.unwrap()(state, (*x.clone(), *y.clone()));
                    }
                    match (*x.clone(), *y.clone()) {
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

type UnaryOpCallback<'a, T> = fn(State<'a>, T) -> Option<(State<'a>, Token<'a>)>;

#[derive(Clone)]
pub struct UnaryOperation<'a> {
    number_fn: Option<UnaryOpCallback<'a, f64>>,
    boolean_fn: Option<UnaryOpCallback<'a, bool>>,
    atom_fn: Option<UnaryOpCallback<'a, &'a str>>,
    string_fn: Option<UnaryOpCallback<'a, String>>,
    list_fn: Option<UnaryOpCallback<'a, Vec<Token<'a>>>>,
    any_fn: Option<UnaryOpCallback<'a, Token<'a>>>,
}

impl<'a> UnaryOperation<'a> {
    pub fn new() -> Self {
        UnaryOperation {
            number_fn: None,
            boolean_fn: None,
            atom_fn: None,
            string_fn: None,
            list_fn: None,
            any_fn: None,
        }
    }

    pub fn for_number(&self, func: UnaryOpCallback<'a, f64>) -> Self {
        let mut op = self.clone();
        op.number_fn = Some(func);

        op
    }

    pub fn for_boolean(&self, func: UnaryOpCallback<'a, bool>) -> Self {
        let mut op = self.clone();
        op.boolean_fn = Some(func);

        op
    }

    pub fn for_atom(&self, func: UnaryOpCallback<'a, &'a str>) -> Self {
        let mut op = self.clone();
        op.atom_fn = Some(func);

        op
    }

    pub fn for_string(&self, func: UnaryOpCallback<'a, String>) -> Self {
        let mut op = self.clone();
        op.string_fn = Some(func);

        op
    }

    pub fn for_list(&self, func: UnaryOpCallback<'a, Vec<Token<'a>>>) -> Self {
        let mut op = self.clone();
        op.list_fn = Some(func);

        op
    }

    pub fn for_any(&self, func: UnaryOpCallback<'a, Token<'a>>) -> Self {
        let mut op = self.clone();
        op.any_fn = Some(func);

        op
    }

    pub fn exec(&self, state: State<'a>, arg: Token<'a>) -> Option<(State<'a>, Token<'a>)> {
        if let Token::List(lst) = arg {
            match &lst[..] {
                [_, Token::Variable(_, Some(x))] => {
                    if self.any_fn != None {
                        return self.any_fn.unwrap()(state, *x.clone());
                    }
                    match *x.clone() {
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

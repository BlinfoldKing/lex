use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::modules::{core::Core, Module};
use crate::utils::tree::Node;

#[derive(Clone)]
pub struct State<'a>(Node<Token<'a>, Definition<'a>>);
impl<'a> State<'a> {
    pub fn new() -> Self {
        Self(Node::new())
    }

    pub fn add(&self, token: Token<'a>, definition: Definition<'a>) -> Self {
        let Self(node) = self;
        let mut new = node.clone();
        match token {
            Token::List(l) => new.push(l, Some(definition)),
            t => new.push(vec![t], Some(definition)),
        }
        Self(new)
    }

    pub fn query(&self, token: Token<'a>) -> Vec<Token<'a>> {
        let Self(node) = self;

        let (_, res) = match token.clone() {
            Token::List(l) => node.find_all(l),
            t => node.find_all(vec![t]),
        };

        res.into_iter()
            .map(|n| n.clone().data.unwrap().inp_sig)
            .collect()
    }

    pub fn exec(&self, token: Token<'a>) -> Option<(Self, Token<'a>)> {
        let Self(node) = self;
        let new = node.clone();

        let found = match token.clone() {
            Token::List(l) => new.find(l),
            t => new.find(vec![t]),
        };

        let cb = match found {
            Some((_, n)) => n.data.clone(),
            None => None,
        };

        match cb {
            Some(def) => Some(def.handle(self.clone(), token)),
            _ => None,
        }
    }

    pub fn load_module<T>(&mut self, module: T) -> Self
    where
        T: Module<'a>,
    {
        let mut res = self.clone();
        for def in module.load() {
            res = res.add(def.clone().inp_sig, def);
        }
        res
    }

    fn debug_state(&self) -> Vec<(Token<'a>, Option<Definition<'a>>)> {
        let Self(node) = self;
        let mut result = vec![];

        for key in node.keys() {
            if let Some((_, n)) = node.find(key.clone()) {
                result.push((Token::List(key), n.data));
            }
        }

        result
    }
}

impl<'a> std::fmt::Debug for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{\n").unwrap();
        for (key, def) in self.debug_state() {
            write!(f, "\t{:?}: {:?}\n", key, def).unwrap();
        }
        write!(f, "}}")
    }
}

pub fn init<'a>() -> State<'a> {
    let mut r = State::new();
    r = r.load_module(Core {});
    r
}

#[test]
fn should_be_able_to_add_number() {
    let r = init();

    let (_, res) = r
        .exec(Token::List(vec![
            Token::Operator("+"),
            Token::Number(3 as f64),
            Token::Number(3 as f64),
        ]))
        .unwrap();

    assert_eq!(Token::Number(6 as f64), res)
}

#[test]
fn should_be_able_to_declare() {
    let r = init();
    let (r1, _) = r
        .exec(Token::List(vec![
            Token::Keyword("dec"),
            Token::List(vec![Token::Atom("hello"), Token::Atom("world")]),
        ]))
        .unwrap();

    let (_, res) = r1
        .exec(Token::List(vec![
            Token::Atom("hello"),
            Token::Atom("world"),
        ]))
        .unwrap();

    assert_eq!(Token::Boolean(true), res)
}

#[test]
fn should_be_able_to_query() {
    let r = init();
    let (r1, _) = r
        .exec(Token::List(vec![
            Token::Keyword("dec"),
            Token::List(vec![Token::Atom("hello"), Token::Atom("world")]),
        ]))
        .unwrap();
    let (r2, _) = r1
        .exec(Token::List(vec![
            Token::Keyword("dec"),
            Token::List(vec![Token::Atom("hello"), Token::Atom("alien")]),
        ]))
        .unwrap();

    let (_, res) = r2
        .exec(Token::List(vec![
            Token::Operator("?"),
            Token::List(vec![Token::Atom("hello"), Token::Variable("X", None)]),
        ]))
        .unwrap();

    assert_eq!(
        Token::List(vec![
            Token::List(vec![Token::Atom("hello"), Token::Atom("world")]),
            Token::List(vec![Token::Atom("hello"), Token::Atom("alien")]),
        ]),
        res
    )
}

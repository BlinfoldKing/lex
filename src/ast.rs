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

    pub fn exec(&self, token: Token<'a>) -> (Self, Option<Token<'a>>) {
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

        let res = match cb {
            Some(def) => def.handle(token),
            _ => None,
        };
        (Self(new), res)
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
}

pub fn root<'a>() -> State<'a> {
    let mut r = State::new();
    r = r.load_module(Core {});
    r
}

#[test]
fn should_be_able_to_add_number() {
    let r = root();

    let (_, res) = r.exec(Token::List(vec![
        Token::Operator("+"),
        Token::Number(3 as f64),
        Token::Number(3 as f64),
    ]));

    assert_eq!(Some(Token::Number(6 as f64)), res)
}

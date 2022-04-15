use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::modules::{core::Core, Import, Module};
use crate::utils::tree::Node;

#[derive(Clone)]
pub struct State(Node<Token, Definition>);
impl State {
    pub fn new() -> Self {
        Self(Node::new())
    }

    pub fn add(&self, token: Token, definition: Definition) -> Self {
        let Self(node) = self;
        let mut new = node.clone();
        match token {
            Token::List(l) => new.push(l, Some(definition)),
            t => new.push(vec![t], Some(definition)),
        }
        Self(new)
    }

    pub fn query(&self, token: Token) -> Vec<Definition> {
        let Self(node) = self;

        let (_, res) = match token.clone() {
            Token::List(l) => node.find_all(l),
            t => node.find_all(vec![t]),
        };

        res.into_iter().map(|n| n.clone().data.unwrap()).collect()
    }

    pub fn find(&self, token: Token) -> Option<Definition> {
        let Self(node) = self;

        let res = match token.clone() {
            Token::List(l) => node.find(l),
            t => node.find(vec![t]),
        };

        match res {
            None => None,
            Some((_, n)) => n.data,
        }
    }

    pub fn exec(&self, token: Token) -> (Self, Token) {
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
            Some(def) => def.handle(self.clone(), token),
            _ => (self.clone(), token),
        }
    }

    pub fn load_module<T>(&mut self, module: T) -> Self
    where
        T: Module,
    {
        let mut res = self.clone();
        for def in module.load() {
            res = res.add(def.clone().inp_sig, def);
        }
        res
    }

    fn debug_state(&self) -> Vec<(Token, Option<Definition>)> {
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

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{\n").unwrap();
        for (key, def) in self.debug_state() {
            write!(f, "\t{:?}: {:?}\n", key, def).unwrap();
        }
        write!(f, "}}")
    }
}

pub fn init() -> State {
    let mut r = State::new();
    r = r.load_module(Core {});
    r = r.load_module(Import {});
    r
}

#[test]
fn should_be_able_to_add_number() {
    let r = init();

    let (_, res) = r.exec(Token::List(vec![
        Token::Operator("+"),
        Token::Number(3 as f64),
        Token::Number(3 as f64),
    ]));

    assert_eq!(Token::Number(6 as f64), res)
}

#[test]
fn should_be_able_to_declare() {
    let r = init();
    let (r1, _) = r.exec(Token::List(vec![
        Token::Keyword("dec"),
        Token::List(vec![Token::Atom("hello"), Token::Atom("world")]),
    ]));

    let (_, res) = r1.exec(Token::List(vec![
        Token::Atom("hello"),
        Token::Atom("world"),
    ]));

    assert_eq!(Token::Boolean(true), res)
}

#[cfg(test)]
mod test_find {
    use super::*;

    fn setup() -> State {
        let r = init();
        let (r1, _) = r.exec(Token::List(vec![
            Token::Keyword("dec"),
            Token::List(vec![Token::Atom("hello"), Token::Atom("world")]),
        ]));
        let (r2, _) = r1.exec(Token::List(vec![
            Token::Keyword("dec"),
            Token::List(vec![Token::Atom("hello"), Token::Atom("alien")]),
        ]));
        let (r3, _) = r2.exec(Token::List(vec![
            Token::Keyword("def"),
            Token::List(vec![Token::Atom("greet"), Token::Variable("X", None)]),
            Token::List(vec![Token::Atom("hello"), Token::Variable("X", None)]),
        ]));
        r3
    }

    #[test]
    fn should_be_able_to_find() {
        let (_, res) = setup().exec(Token::List(vec![
            Token::Operator("!"),
            Token::List(vec![Token::Atom("hello"), Token::Variable("X", None)]),
        ]));

        assert_eq!(
            Token::List(vec![Token::Atom("hello"), Token::Atom("world")]),
            res
        )
    }

    #[test]
    fn should_be_able_to_find_defined_pattern_with_exact_value() {
        let (_, res) = setup().exec(Token::List(vec![
            Token::Operator("!"),
            Token::List(vec![Token::Atom("greet"), Token::Atom("alien")]),
        ]));

        assert_eq!(
            Token::List(vec![Token::Atom("greet"), Token::Atom("alien")]),
            res
        )
    }

    #[test]
    fn should_be_able_to_find_defined_pattern() {
        let (_, res) = setup().exec(Token::List(vec![
            Token::Operator("!"),
            Token::List(vec![Token::Atom("greet"), Token::Variable("Z", None)]),
        ]));

        assert_eq!(
            Token::List(vec![Token::Atom("greet"), Token::Atom("world")]),
            res
        )
    }

    #[test]
    fn should_be_able_to_exec_defined_pattern() {
        let (_, res) = setup().exec(Token::List(vec![
            Token::Atom("greet"),
            Token::Atom("alien"),
        ]));

        assert_eq!(Token::Boolean(true), res)
    }

    #[test]
    fn should_be_able_to_query() {
        let (_, res) = setup().exec(Token::List(vec![
            Token::Operator("?"),
            Token::List(vec![Token::Atom("hello"), Token::Variable("X", None)]),
        ]));

        assert_eq!(
            Token::List(vec![
                Token::List(vec![Token::Atom("hello"), Token::Atom("world")]),
                Token::List(vec![Token::Atom("hello"), Token::Atom("alien")]),
            ]),
            res
        )
    }

    #[test]
    fn should_be_able_to_query_definition() {
        let (_, res) = setup().exec(Token::List(vec![
            Token::Operator("?"),
            Token::List(vec![Token::Atom("greet"), Token::Variable("Z", None)]),
        ]));

        assert_eq!(
            Token::List(vec![
                Token::List(vec![Token::Atom("greet"), Token::Atom("world")]),
                Token::List(vec![Token::Atom("greet"), Token::Atom("alien")]),
            ]),
            res
        )
    }
}

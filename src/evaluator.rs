use crate::ast::{init, State};
use crate::grammar::token::Token;
use crate::grammar::{document, repl_line};

pub struct Engine {
    state: State,
}

impl Engine {
    pub fn new() -> Self {
        Self { state: init() }
    }

    pub fn parse(&mut self, input: &str) -> Result<Token, ()> {
        let parse = document(input);

        let document: Token;
        match parse {
            Err(_) => return Err(()),
            Ok((_, token)) => document = token,
        };

        let (state, result) = self.state.exec(document);

        self.state = state;
        Ok(result)
    }

    pub fn parse_line(&mut self, input: &str) -> Result<Token, ()> {
        let parse = repl_line(input);

        let document: Token;
        match parse {
            Err(_) => return Err(()),
            Ok((_, token)) => document = token,
        };

        let (state, result) = self.state.exec(document);

        self.state = state;
        Ok(result)
    }
}

use crate::grammar::token::Token;
use crate::grammar::{document, repl_line};
use crate::state::State;

pub struct Engine {
    state: State,
}

impl Engine {
    pub fn new() -> Self {
        let mut res = Self {
            state: State::new(),
        };

        res.state.load(crate::stdlib::Std);

        res
    }

    pub fn parse(&mut self, input: &str) -> Result<Token, ()> {
        let parse = document(input);

        let document: Token;
        match parse {
            Err(_) => return Err(()),
            Ok((_, token)) => document = token,
        };

        let result = self.state.exec(document);
        if let Some(value) = self.state.return_value() {
            return Ok(value);
        }

        Ok(result)
    }

    pub fn parse_line(&mut self, input: &str) -> Result<Token, ()> {
        let parse = repl_line(input);

        let document: Token;
        match parse {
            Err(_) => return Err(()),
            Ok((_, token)) => document = token,
        };

        let result = self.state.exec(document);

        Ok(result)
    }
}

use crate::ast::{init, Scope};
use crate::grammar::token::Token;
use crate::grammar::{document, repl_line};

pub struct Engine {
    scope: Scope,
}

impl Engine {
    pub fn new() -> Self {
        Self { scope: init() }
    }

    pub fn parse(&mut self, input: &str) -> Result<Token, ()> {
        let parse = document(input);

        let document: Token;
        match parse {
            Err(_) => return Err(()),
            Ok((_, token)) => document = token,
        };

        let (scope, result) = self.scope.exec(document);

        self.scope = scope;
        Ok(result)
    }

    pub fn parse_line(&mut self, input: &str) -> Result<Token, ()> {
        let parse = repl_line(input);

        let document: Token;
        match parse {
            Err(_) => return Err(()),
            Ok((_, token)) => document = token,
        };

        let (scope, result) = self.scope.exec(document);

        self.scope = scope;
        Ok(result)
    }
}

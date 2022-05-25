use crate::definition::Definition;
use crate::grammar::token::Token;
use crate::modules::Module;
use crate::utils::arraymap::ArrayMap;
use crate::utils::posibility::Posibility;
use std::collections::HashMap;

// for debugging and error tracing
static mut DEPTH: i32 = 1;
static mut DEBUG: bool = false;

#[derive(Clone, Debug)]
pub struct State {
    parent: Option<*mut Self>,
    definitions: ArrayMap<Token, Definition>,
    return_value: Option<Token>,
    variables: HashMap<String, Token>,
    wildcards: HashMap<String, Posibility<Token>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            parent: None,
            definitions: ArrayMap::new(),
            return_value: None,
            variables: HashMap::new(),
            wildcards: HashMap::new(),
        }
    }

    pub fn with_parent(&self, parent: *mut Self) -> Self {
        let mut res = self.clone();
        res.parent = Some(parent);
        res
    }

    pub fn find_all(&self, token: Token) -> Vec<Definition> {
        // find local matches first if any
        let found = self.definitions.search(token.clone());

        if found.len() > 0 {
            return found;
        }

        if let Some(parent) = self.parent {
            let found = unsafe { (*parent).find_all(token) };
            return found;
        }

        vec![]
    }

    pub fn run(&mut self, token: Token) -> Token {
        let mut res: Vec<Token> = vec![];

        let defs = self.find_all(token.clone());
        for def in defs {
            let result = def.handle(&mut self.clone(), token.clone());
            res.push(result);
        }

        if res.len() == 0 {
            return token;
        } else if res.len() == 1 {
            return res[0].clone();
        }

        Token::List(res)
    }

    pub fn exec(&mut self, token: Token) -> Token {
        self.push_trace(token.clone());
        let mut state = Self::new().with_parent(self);

        let mut token = token;
        match token {
            Token::Document(_, content) => token = state.exec(*content.clone()),
            Token::List(list) => {
                let mut res = vec![];

                for item in list {
                    let out = state.exec(item);
                    if state.return_value != None {
                        return state.return_value.unwrap();
                    }
                    res.push(out);
                }

                token = Token::List(res)
            }
            Token::Definition(_, _) => (),
            token => return token,
        };

        let res = self.run(token.clone());
        // let found = state.find(token.clone());

        // let res = match found {
        //     Some(def) => def.handle(&mut state, token),
        //     None => token,
        // };

        self.pop_trace(res.clone());

        res
    }

    pub fn add(&mut self, token: Token, definition: Definition) {
        self.definitions.push(token, definition)
    }

    pub fn load<T: Module>(&mut self, module: T) {
        let defs = module.load();
        for def in defs {
            self.add(def.inp_sig.clone(), def);
        }
    }

    pub fn include<T: Module>(&mut self, module: T) {
        if let Some(parent) = self.parent {
            let defs = module.load();
            for def in defs {
                unsafe { (*parent).add(def.inp_sig.clone(), def) };
            }
        }
    }

    pub fn add_to_parent(&mut self, token: Token, definition: Definition) {
        if let Some(parent) = self.parent {
            unsafe { (*parent).add(token, definition) };
        }
    }

    pub fn _return(&mut self, value: Token) {
        if let Some(parent) = self.parent {
            unsafe { (*parent).return_value = Some(value) };
        }
    }

    fn push_trace(&mut self, token: Token) {
        unsafe {
            match token {
                Token::List(_) => self.print_trace("in", token),
                Token::Document(_, _) => self.print_trace("in", token),
                Token::Definition(_, _) => self.print_trace("in", token),
                _ => (),
            }

            DEPTH += 1;
        }
    }

    fn pop_trace(&mut self, token: Token) {
        unsafe {
            DEPTH -= 1;

            match token {
                Token::List(_) => self.print_trace("out", token),
                Token::Document(_, _) => self.print_trace("out", token),
                Token::Definition(_, _) => self.print_trace("out", token),
                _ => (),
            }
        }
    }

    fn print_trace(&self, prefix: &str, token: Token) {
        unsafe {
            if DEBUG {
                for _ in 1..DEPTH {
                    print!("\t");
                }
                println!("{} {}", prefix, token);
            }
        }
    }

    pub fn return_value(&self) -> Option<Token> {
        self.return_value.clone()
    }
}

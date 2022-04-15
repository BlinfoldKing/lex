use crate::ast::State;
use crate::grammar::token::Token;
use std::sync::Arc;

pub type Handler = Arc<dyn Fn(State, Token) -> (State, Token) + Send + Sync>;

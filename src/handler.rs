use crate::ast::Scope;
use crate::grammar::token::Token;
use std::sync::Arc;

pub type Handler = Arc<dyn Fn(Scope, Token) -> (Scope, Token) + Send + Sync>;

use crate::ast::State;
use crate::grammar::token::Token;
use std::sync::Arc;

pub type Handler<'a> = Arc<dyn Fn(State<'a>, Token<'a>) -> (State<'a>, Token<'a>) + Send + Sync>;

/*
 * (+ X Y) // token [+ (X, None) (Y, None)]
 * (+ 1 1)
 */

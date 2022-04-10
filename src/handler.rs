use crate::grammar::token::Token;
use std::sync::Arc;

pub type Handler<'a> = Arc<Box<dyn Fn(Token<'a>) -> Token<'a> + Send + Sync>>;

#[macro_export]
macro_rules! handler {
    ($f: expr) => {
        Arc::new(Box::new($f))
    };
}

/*
 * (+ X Y) // token [+ (X, None) (Y, None)]
 * (+ 1 1)
 */

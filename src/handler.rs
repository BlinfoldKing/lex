use crate::grammar::token::Token;
use crate::state::State;

pub type Handler = dyn Fn(&mut State, Token) -> Token;

// impl Handler {
//     pub fn new(func: fn(&State, Token) -> Token) -> Self {
//         Self { func }
//     }
// }

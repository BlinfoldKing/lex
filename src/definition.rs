use crate::grammar::token::Token;
use crate::handler::Handler;
use crate::state::State;
use crate::utils::variables;
use std::sync::Arc;

#[derive(Clone)]
pub struct Definition {
    pub inp_sig: Token,
    pub out_sig: Token,
    pub res_sig: Token,
    pub func: Arc<Box<Handler>>,
}

impl Definition {
    pub fn handle(&self, state: &mut State, inp: Token) -> Token {
        let arg = variables::fill_variable(inp.clone(), self.inp_sig.clone());
        let func = &self.func;

        let out = func(state, arg);
        out
    }
}

impl std::fmt::Debug for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} -> {}",
            self.inp_sig, self.out_sig, self.res_sig
        )
    }
}

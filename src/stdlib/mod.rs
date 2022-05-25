pub mod core;
pub mod fmt;
pub mod import;
pub mod io;
pub mod math;
pub mod os;

use crate::definition::Definition;
use crate::modules::Module;
use std::collections::HashMap;

pub struct Std;

impl Module for Std {
    fn load(&self) -> Vec<Definition> {
        let mut res = vec![];

        let mut hm: HashMap<String, Box<dyn Module>> = HashMap::new();
        hm.insert("fmt".to_owned(), Box::new(fmt::Fmt));
        let import = import::Import::new(hm);
        res.extend(import.load());

        res.extend(core::Core.load());

        res
    }
}

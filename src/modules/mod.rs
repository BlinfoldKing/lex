use crate::definition::Definition;

pub mod core;

pub trait Module<'a> {
    fn load(&self) -> Vec<Definition<'a>>;
}

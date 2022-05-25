use crate::definition::Definition;

pub trait Module {
    fn load(&self) -> Vec<Definition>;
}

impl Module for &dyn Module {
    fn load(&self) -> Vec<Definition> {
        let m = *self;
        m.load()
    }
}

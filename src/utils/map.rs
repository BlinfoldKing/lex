#[derive(Debug, PartialEq, Clone)]
pub struct Map<U, V> {
    store: Vec<(U, V)>,
}

impl<U, V> Map<U, V>
where
    U: std::cmp::PartialEq + std::fmt::Debug,
{
    pub fn new() -> Self {
        Map { store: vec![] }
    }

    pub fn get(&self, input: U) -> Option<&V> {
        for (key, value) in self.store.iter() {
            if input == *key {
                return Some(value);
            }
        }

        return None;
    }

    pub fn insert(&mut self, key: U, value: V) {
        self.store.push((key, value))
    }
}

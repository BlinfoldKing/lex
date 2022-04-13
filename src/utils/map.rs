#[derive(Debug, PartialEq, Clone)]
pub struct Map<U, V> {
    store: Vec<(U, V)>,
}

impl<U, V> Map<U, V>
where
    U: Clone + std::cmp::PartialEq + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
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

    pub fn get_index(&self, input: U) -> Option<usize> {
        for (i, (key, _)) in self.store.iter().enumerate() {
            if input == *key {
                return Some(i);
            }
        }

        return None;
    }

    pub fn insert(&mut self, key: U, value: V) {
        self.store.push((key, value))
    }

    pub fn upsert(&mut self, key: U, value: V) {
        match self.get_index(key.clone()) {
            None => self.insert(key, value),
            Some(index) => {
                let _ = std::mem::replace(&mut self.store[index], (key, value));
            }
        }
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn key_values(&self) -> (Vec<U>, Vec<V>) {
        let mut keys: Vec<U> = vec![];
        let mut values: Vec<V> = vec![];
        for (key, value) in &self.store {
            keys.push(key.clone());
            values.push(value.clone());
        }

        (keys, values)
    }

    pub fn get_all(&self) -> Vec<(U, V)> {
        self.store.clone()
    }

    pub fn extend(&mut self, input: Map<U, V>) {
        self.store.extend(input.get_all().into_iter())
    }
}

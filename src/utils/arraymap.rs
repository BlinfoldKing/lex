#[derive(Debug, Clone)]
pub struct ArrayMap<Key, Val> {
    store: Vec<(Key, Val)>,
}

impl<Key, Val> ArrayMap<Key, Val>
where
    Key: PartialEq + core::fmt::Debug,
    Val: Clone,
{
    pub fn new() -> Self {
        Self { store: vec![] }
    }

    pub fn get(&self, key: Key) -> Option<Val> {
        for item in &self.store {
            let (k, v) = item;
            if *k == key {
                return Some(v.clone());
            }
        }

        None
    }

    pub fn search(&self, key: Key) -> Vec<Val> {
        let mut res = vec![];
        for item in &self.store {
            let (k, v) = item;
            if *k == key {
                res.push(v.clone());
            }
        }

        res
    }

    pub fn push(&mut self, key: Key, val: Val) {
        self.store.push((key, val))
    }
}

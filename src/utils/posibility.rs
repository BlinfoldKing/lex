#[derive(Debug, Clone, Eq)]
pub struct Posibility<T> {
    store: Vec<T>,
}

impl<T> Posibility<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self { store: vec![] }
    }

    pub fn with_value(&self, value: T) -> Self {
        let mut ret = self.clone();
        ret.store.push(value);

        ret.clone()
    }

    pub fn values(&self) -> Vec<T> {
        self.store.clone()
    }
}

impl<T> PartialEq for Posibility<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i1 in self.store.iter() {
            for i2 in other.store.iter() {
                if i1 == i2 {
                    return true;
                }
            }
        }
        return false;
    }
}

impl<T> PartialEq<T> for Posibility<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        for i in self.store.iter() {
            if i == other {
                return true;
            }
        }

        return false;
    }
}

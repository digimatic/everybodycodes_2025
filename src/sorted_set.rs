#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SortedSet<T: Ord>(Vec<T>);
impl<T: Ord> SortedSet<T> {
    pub fn new() -> Self {
        SortedSet(Vec::new())
    }

    pub fn insert(&mut self, value: T) -> bool {
        match self.0.binary_search(&value) {
            Ok(_) => false, // already exists
            Err(pos) => {
                self.0.insert(pos, value);
                true
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.0.binary_search(value).is_ok()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        match self.0.binary_search(value) {
            Ok(pos) => {
                self.0.remove(pos);
                true
            }
            Err(_) => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Ord> IntoIterator for SortedSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T: Ord> IntoIterator for &'a SortedSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T: Ord> SortedSet<T> {
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T: Ord> From<Vec<T>> for SortedSet<T> {
    fn from(mut vec: Vec<T>) -> Self {
        vec.sort();
        vec.dedup();
        SortedSet(vec)
    }
}

impl<T: Ord + Clone> From<&[T]> for SortedSet<T> {
    fn from(slice: &[T]) -> Self {
        let mut vec = slice.to_vec();
        vec.sort();
        vec.dedup();
        SortedSet(vec)
    }
}

impl<T: Ord + Clone> From<&std::collections::HashSet<T>> for SortedSet<T> {
    fn from(set: &std::collections::HashSet<T>) -> Self {
        let mut vec: Vec<T> = set.iter().cloned().collect();
        vec.sort();
        SortedSet(vec)
    }
}

impl<T: Ord> Default for SortedSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

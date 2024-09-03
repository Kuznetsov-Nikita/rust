#![forbid(unsafe_code)]

use std::{borrow::Borrow, iter::FromIterator, ops::Index};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

impl<K: Ord, V> FlatMap<K, V> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let insert_pos = self.0.binary_search_by_key(&&key, |(first, _second)| first);

        match insert_pos {
            Ok(pos) => {
                let previous_value = self.0.remove(pos).1;
                self.0.insert(pos, (key, value));
                Some(previous_value)
            }
            Err(pos) => {
                self.0.insert(pos, (key, value));
                None
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let pos = self
            .0
            .binary_search_by_key(&key, |(first, _second)| first.borrow());

        match pos {
            Ok(pos) => Some(&self.0[pos].1),
            Err(_pos) => None,
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let pos = self
            .0
            .binary_search_by_key(&key, |(first, _second)| first.borrow());

        match pos {
            Ok(pos) => Some(self.0.remove(pos).1),
            Err(_pos) => None,
        }
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let pos = self
            .0
            .binary_search_by_key(&key, |(first, _second)| first.borrow());

        match pos {
            Ok(pos) => Some(self.0.remove(pos)),
            Err(_pos) => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<Q, K: Ord, V> Index<&Q> for FlatMap<K, V>
where
    K: Borrow<Q>,
    Q: Ord + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).unwrap()
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
    {
        for elem in iter {
            self.insert(elem.0, elem.1);
        }
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for FlatMap<K, V> {
    fn from(value: Vec<(K, V)>) -> Self {
        let mut new_flatmap = FlatMap::new();

        for elem in value {
            new_flatmap.insert(elem.0, elem.1);
        }

        new_flatmap
    }
}

impl<K: Ord, V> From<FlatMap<K, V>> for Vec<(K, V)> {
    fn from(value: FlatMap<K, V>) -> Self {
        let mut new_vec = vec![];

        for elem in value.0 {
            new_vec.push(elem);
        }

        new_vec
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        let mut new_flatmap = FlatMap::new();

        for elem in iter {
            new_flatmap.insert(elem.0, elem.1);
        }

        new_flatmap
    }
}

impl<K: Ord, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

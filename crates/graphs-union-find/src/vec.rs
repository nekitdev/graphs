use std::cmp::Ordering;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::{
    key::{DefaultKey, Error, Key, KeyError, check_index},
    rank::{Rank, ZERO},
};

pub const NEW: &str = "failed to create union-find";
pub const SET: &str = "failed to create new set";
pub const FIND: &str = "failed to find set representative";
pub const SAME_SET: &str = "failed to check if keys are in the same set";
pub const UNION: &str = "failed to union sets";
pub const INTO_KEYS: &str = "failed to convert union-find into keys";

pub struct UnionFind<K: Key = DefaultKey> {
    parent: Vec<K>,
    rank: Vec<Rank>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Union {
    Already,
    Unified,
}

pub use Union::{Already, Unified};

impl Union {
    pub const fn is_already(self) -> bool {
        matches!(self, Self::Already)
    }

    pub const fn is_unified(self) -> bool {
        matches!(self, Self::Unified)
    }
}

impl<K: Key> UnionFind<K> {
    pub const fn empty() -> Self {
        let parent = Vec::new();
        let rank = Vec::new();

        Self::construct(parent, rank)
    }

    const fn construct(parent: Vec<K>, rank: Vec<Rank>) -> Self {
        Self { parent, rank }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let parent = Vec::with_capacity(capacity);
        let rank = Vec::with_capacity(capacity);

        Self::construct(parent, rank)
    }

    pub const fn count(&self) -> usize {
        self.parent.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    pub fn try_new(count: usize) -> Result<Self, KeyError> {
        let parent = (0..count).map(K::try_new).collect::<Result<_, _>>()?;

        let rank = vec![ZERO; count];

        Ok(Self::construct(parent, rank))
    }

    pub fn new(count: usize) -> Self {
        Self::try_new(count).expect(NEW)
    }

    pub fn try_new_set(&mut self) -> Result<K, KeyError> {
        let key = K::try_new(self.count())?;

        self.push_new(key);

        Ok(key)
    }

    pub fn new_set(&mut self) -> K {
        self.try_new_set().expect(SET)
    }

    fn push_new(&mut self, key: K) {
        self.parent.push(key);
        self.rank.push(ZERO);
    }

    pub fn try_find(&self, mut key: K) -> Result<K, Error<K>> {
        let mut key_index = key.try_unkey()?;

        let count = self.count();

        check_index(key_index, count)?;

        loop {
            // SAFETY: internal keys are trusted to be valid
            let parent = unsafe { get_unchecked_copy(&self.parent, key_index) };

            if parent == key {
                break;
            }

            key = parent;
            key_index = parent.try_unkey()?;
        }

        Ok(key)
    }

    pub fn find(&self, key: K) -> K {
        self.try_find(key).expect(FIND)
    }

    pub fn try_find_mut(&mut self, mut key: K) -> Result<K, Error<K>> {
        let mut key_index = key.try_unkey()?;

        let count = self.count();

        check_index(key_index, count)?;

        // NOTE: this method performs path compression

        // SAFETY: internal keys are trusted to be valid
        let mut parent = unsafe { get_unchecked_copy(&self.parent, key_index) };

        let mut parent_index = parent.try_unkey()?;

        while parent != key {
            // SAFETY: same as above
            let grandparent = unsafe { get_unchecked_copy(&self.parent, parent_index) };

            // SAFETY: same as above, and path compression is trusted to be correct
            unsafe {
                set_unchecked(&mut self.parent, key_index, grandparent);
            }

            key = parent;
            key_index = parent_index;
            parent = grandparent;
            parent_index = grandparent.try_unkey()?;
        }

        Ok(key)
    }

    pub fn find_mut(&mut self, key: K) -> K {
        self.try_find_mut(key).expect(FIND)
    }

    pub fn try_same_set(&self, one: K, two: K) -> Result<bool, Error<K>> {
        let one_repr = self.try_find(one)?;
        let two_repr = self.try_find(two)?;

        Ok(one_repr == two_repr)
    }

    pub fn same_set(&self, one: K, two: K) -> bool {
        self.try_same_set(one, two).expect(SAME_SET)
    }

    pub fn try_union(&mut self, one: K, two: K) -> Result<Union, Error<K>> {
        if one == two {
            return Ok(Already);
        }

        let one_repr = self.try_find_mut(one)?;
        let two_repr = self.try_find_mut(two)?;

        if one_repr == two_repr {
            return Ok(Already);
        }

        let one_repr_index = one_repr.try_unkey()?;
        let two_repr_index = two_repr.try_unkey()?;

        // TODO: use `unsafe` here?

        let one_rank = self.rank[one_repr_index];
        let two_rank = self.rank[two_repr_index];

        match one_rank.cmp(&two_rank) {
            Ordering::Less => self.parent[one_repr_index] = two_repr,
            Ordering::Greater => self.parent[two_repr_index] = one_repr,
            Ordering::Equal => {
                self.parent[two_repr_index] = one_repr;
                self.rank[one_repr_index] += 1;
            }
        }

        Ok(Unified)
    }

    pub fn union(&mut self, one: K, two: K) -> Union {
        self.try_union(one, two).expect(UNION)
    }

    pub fn try_into_keys(mut self) -> Result<Vec<K>, Error<K>> {
        for index in 0..self.count() {
            let key = unsafe { get_unchecked_copy(&self.parent, index) };

            let key_repr = self.try_find_mut(key)?;

            unsafe {
                set_unchecked(&mut self.parent, index, key_repr);
            }
        }

        Ok(self.parent)
    }

    pub fn into_keys(self) -> Vec<K> {
        self.try_into_keys().expect(INTO_KEYS)
    }
}

pub(crate) const fn copy<T: Copy>(value: &T) -> T {
    *value
}

#[inline]
pub(crate) unsafe fn get_unchecked_copy<K: Key>(keys: &[K], index: usize) -> K {
    debug_assert!(index < keys.len());

    copy(unsafe { keys.get_unchecked(index) })
}

#[inline]
pub(crate) unsafe fn set_unchecked<K: Key>(keys: &mut [K], index: usize, key: K) {
    debug_assert!(index < keys.len());

    let mutable = unsafe { keys.get_unchecked_mut(index) };

    *mutable = key;
}

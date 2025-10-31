use core::{fmt, marker::PhantomData};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use graphs_core::keys::{DefaultUntypedKey, Key};

use crate::rank::Rank;

pub struct UnionFind<K: Key = DefaultUntypedKey> {
    parent: Vec<K>,
    rank: Vec<Rank>,
}

pub const NEW: &str = "failed to create union-find";
pub const SET: &str = "failed to append new set";
pub const FIND: &str = "failed to find representative";

pub trait UnionFindMethods: Sized {
    type Key: Key;

    fn empty() -> Self;

    // fn try_new(count: usize) -> Result<Self, IndexError<Self::Key>>;

    // fn try_new_set(&mut self) -> Result<Self::Key, IndexError<Self::Key>>;

    // fn try_find(&self, key: Self::Key) -> Result<Self::Key, Self::Error>;

    // fn try_find_mut(&mut self, key: Self::Key) -> Result<Self::Key, Self::Error>;

    // fn try_same_set(&self, one: Self::Key, two: Self::Key) -> Result<bool, Self::Error> {
    //     let one_repr = self.try_find(one)?;
    //     let two_repr = self.try_find(two)?;

    //     Ok(one_repr == two_repr)
    // }

    // fn new(count: usize) -> Self {
    //     Self::try_new(count).expect(NEW)
    // }

    // fn new_set(&mut self) -> Self::Key {
    //     self.try_new_set().expect(SET)
    // }

    // fn find(&self, key: Self::Key) -> Self::Key {
    //     self.try_find(key).expect(FIND)
    // }

    // fn find_mut(&mut self, key: Self::Key) -> Self::Key {
    //     self.try_find_mut(key).expect(FIND)
    // }

    // fn same_set(&self, one: Self::Key, two: Self::Key) -> bool {
    //     self.find(one) == self.find(two)
    // }
}

impl<K: Key> UnionFind<K> {
    pub const fn empty() -> Self {
        let parent = Vec::new();
        let rank = Vec::new();

        Self::construct(parent, rank)
    }

    const fn construct(parent: Vec<K>, rank: Vec<u8>) -> Self {
        Self { parent, rank }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let parent = Vec::with_capacity(capacity);
        let rank = Vec::with_capacity(capacity);

        Self::construct(parent, rank)
    }

    pub const fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.parent.len()
    }
}

impl<K: Key> UnionFindMethods for UnionFind<K> {
    type Key = K;

    fn empty() -> Self {
        Self::empty()
    }
}

// impl<K: Index> UnionFind<K> {
//     pub const fn empty() -> Self {
//         Self::construct(Vec::new(), Vec::new())
//     }

//     pub fn with_capacity(capacity: usize) -> Self {
//         let parent = Vec::with_capacity(capacity);
//         let rank = Vec::with_capacity(capacity);

//         Self::construct(parent, rank)
//     }

//     const fn construct(parent: Vec<K>, rank: Vec<u8>) -> Self {
//         Self { parent, rank }
//     }

//     pub const fn len(&self) -> usize {
//         self.parent.len()
//     }

//     pub const fn is_empty(&self) -> bool {
//         self.parent.is_empty()
//     }

//     pub fn new(count: usize) -> Self {
//         let parent = (0..count).map(K::of).collect();

//         let rank = vec![0; count];

//         Self::construct(parent, rank)
//     }

//     fn push_new(&mut self, key: K) {
//         self.parent.push(key);
//         self.rank.push(0);
//     }

//     pub fn try_new_set(&mut self) -> Result<K, KeyError<K>> {
//         let key = K::try_of(self.len());

//         self.push_new(key);

//         Some(key)
//     }

//     pub fn new_set(&mut self) -> K {
//         let key = K::of(self.len());

//         self.push_new(key);

//         key
//     }

//     pub fn find(&self, key: K) -> K {
//         todo!()
//     }

//     pub fn find_mut(&mut self, key: K) -> K {
//         todo!()
//     }
// }

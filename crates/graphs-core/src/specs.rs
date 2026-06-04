use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{
    kinds::{DefaultKind, Kind},
    loops::{DefaultLoop, Loop},
    types::{DefaultType, Type},
};

pub struct Specs<K: Kind = DefaultKind, T: Type = DefaultType, L: Loop = DefaultLoop> {
    kind_spec: PhantomData<K>,
    type_spec: PhantomData<T>,
    loop_spec: PhantomData<L>,
}

impl<K: Kind, T: Type, L: Loop> Clone for Specs<K, T, L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K: Kind, T: Type, L: Loop> Copy for Specs<K, T, L> {}

impl<K: Kind, T: Type, L: Loop> PartialEq for Specs<K, T, L> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<K: Kind, T: Type, L: Loop> Eq for Specs<K, T, L> {}

impl<K: Kind, T: Type, L: Loop> PartialOrd for Specs<K, T, L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Kind, T: Type, L: Loop> Ord for Specs<K, T, L> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<K: Kind, T: Type, L: Loop> Default for Specs<K, T, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Kind, T: Type, L: Loop> Hash for Specs<K, T, L> {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // nothing to hash
    }
}

impl<K: Kind, T: Type, L: Loop> Specs<K, T, L> {
    pub const fn new() -> Self {
        Self {
            kind_spec: PhantomData,
            type_spec: PhantomData,
            loop_spec: PhantomData,
        }
    }
}

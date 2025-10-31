use core::fmt;

use crate::id::{EdgeId, EdgeType, EdgeTypeId, Id, NodeId, NodeType, NodeTypeId, UntypedId};

pub const OF: &str = "can not convert index to key";
pub const INDEX: &str = "can not convert key to index";

pub trait Key: Id {
    fn try_of(index: usize) -> Option<Self>;
    fn try_index(self) -> Option<usize>;

    fn of(index: usize) -> Self {
        Self::try_of(index).expect(OF)
    }

    fn index(self) -> usize {
        self.try_index().expect(INDEX)
    }
}

pub trait UntypedKey: Key + UntypedId {}
pub trait NodeTypeKey: Key + NodeTypeId {}
pub trait EdgeTypeKey: Key + EdgeTypeId {}

impl<K: Key + UntypedId> UntypedKey for K {}
impl<N: Key + NodeTypeId> NodeTypeKey for N {}
impl<E: Key + EdgeTypeId> EdgeTypeKey for E {}

macro_rules! impl_untyped_key {
    ($($int: ty),* $(,)?) => {
        $(
            impl $crate::keys::Key for $int {
                fn try_of(index: usize) -> Option<Self> {
                    index.try_into().ok()
                }

                fn try_index(self) -> Option<usize> {
                    self.try_into().ok()
                }
            }
        )*
    };
}

impl_untyped_key!(u8, u16, u32, u64, u128, usize);

pub type DefaultUntypedKey = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NodeKey<K: UntypedKey = DefaultUntypedKey> {
    inner: K,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EdgeKey<K: UntypedKey = DefaultUntypedKey> {
    inner: K,
}

impl<K: UntypedKey> From<NodeId<K>> for NodeKey<K> {
    fn from(id: NodeId<K>) -> Self {
        Self::new(id.get())
    }
}

impl<K: UntypedKey> From<EdgeId<K>> for EdgeKey<K> {
    fn from(id: EdgeId<K>) -> Self {
        Self::new(id.get())
    }
}

impl<K: UntypedKey> From<NodeKey<K>> for NodeId<K> {
    fn from(key: NodeKey<K>) -> Self {
        Self::new(key.get())
    }
}

impl<K: UntypedKey> From<EdgeKey<K>> for EdgeId<K> {
    fn from(key: EdgeKey<K>) -> Self {
        Self::new(key.get())
    }
}

impl<K: UntypedKey> Default for NodeKey<K> {
    fn default() -> Self {
        Self::limit()
    }
}

impl<K: UntypedKey> Default for EdgeKey<K> {
    fn default() -> Self {
        Self::limit()
    }
}

impl<K: UntypedKey + fmt::Display> fmt::Display for NodeKey<K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<K: UntypedKey + fmt::Display> fmt::Display for EdgeKey<K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<K: UntypedKey> Id for NodeKey<K> {
    type Type = NodeType;

    const LIMIT: Self = Self::limit();
}

impl<K: UntypedKey> Id for EdgeKey<K> {
    type Type = EdgeType;

    const LIMIT: Self = Self::limit();
}

impl<K: UntypedKey> Key for NodeKey<K> {
    fn try_of(index: usize) -> Option<Self> {
        K::try_of(index).map(Self::new)
    }

    fn try_index(self) -> Option<usize> {
        self.get().try_index()
    }

    fn of(index: usize) -> Self {
        Self::new(K::of(index))
    }

    fn index(self) -> usize {
        self.get().index()
    }
}

impl<K: UntypedKey> Key for EdgeKey<K> {
    fn try_of(index: usize) -> Option<Self> {
        K::try_of(index).map(Self::new)
    }

    fn try_index(self) -> Option<usize> {
        self.get().try_index()
    }

    fn of(index: usize) -> Self {
        Self::new(K::of(index))
    }

    fn index(self) -> usize {
        self.get().index()
    }
}

impl<K: UntypedKey> NodeKey<K> {
    pub const fn new(inner: K) -> Self {
        Self { inner }
    }

    pub const fn limit() -> Self {
        Self::new(K::LIMIT)
    }

    pub const fn get(self) -> K {
        self.inner
    }
}

impl<K: UntypedKey> EdgeKey<K> {
    pub const fn new(inner: K) -> Self {
        Self { inner }
    }

    pub const fn limit() -> Self {
        Self::new(K::LIMIT)
    }

    pub const fn get(self) -> K {
        self.inner
    }
}

pub type DefaultNodeKey = NodeKey<DefaultUntypedKey>;
pub type DefaultEdgeKey = EdgeKey<DefaultUntypedKey>;

pub const fn node_key<K: UntypedKey>(key: K) -> NodeKey<K> {
    NodeKey::new(key)
}

pub const fn edge_key<K: UntypedKey>(key: K) -> EdgeKey<K> {
    EdgeKey::new(key)
}

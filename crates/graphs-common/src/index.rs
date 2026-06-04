use core::fmt;

use graphs_core::{
    id::{EdgeType, Id, NodeType},
    index::{DefaultUntypedIndex, Index, UntypedIndex},
    sentinel::Sentinel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NodeIndex<I: UntypedIndex = DefaultUntypedIndex> {
    inner: I,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EdgeIndex<I: UntypedIndex = DefaultUntypedIndex> {
    inner: I,
}

#[cfg(feature = "serde")]
mod serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{EdgeIndex, NodeIndex, UntypedIndex};

    impl<I: UntypedIndex> Serialize for NodeIndex<I> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.get().serialize(serializer)
        }
    }

    impl<'de, I: UntypedIndex> Deserialize<'de> for NodeIndex<I> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let inner = I::deserialize(deserializer)?;

            let node_index = Self::new(inner);

            Ok(node_index)
        }
    }

    impl<I: UntypedIndex> Serialize for EdgeIndex<I> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.get().serialize(serializer)
        }
    }

    impl<'de, I: UntypedIndex> Deserialize<'de> for EdgeIndex<I> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let inner = I::deserialize(deserializer)?;

            let edge_index = Self::new(inner);

            Ok(edge_index)
        }
    }
}

impl<I: UntypedIndex> fmt::Display for NodeIndex<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{untyped}", untyped = self.get())
    }
}

impl<I: UntypedIndex> fmt::Display for EdgeIndex<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{untyped}", untyped = self.get())
    }
}

impl<I: UntypedIndex> Sentinel for NodeIndex<I> {
    const SENTINEL: Self = Self::new(I::SENTINEL);

    fn is_sentinel(&self) -> bool {
        self.get().is_sentinel()
    }
}

impl<I: UntypedIndex> Sentinel for EdgeIndex<I> {
    const SENTINEL: Self = Self::new(I::SENTINEL);

    fn is_sentinel(&self) -> bool {
        self.get().is_sentinel()
    }
}

impl<I: UntypedIndex> Id for NodeIndex<I> {
    type Type = NodeType;
}

impl<I: UntypedIndex> Id for EdgeIndex<I> {
    type Type = EdgeType;
}

impl<I: UntypedIndex> Index for NodeIndex<I> {
    fn try_of(index: usize) -> Option<Self> {
        I::try_of(index).map(Self::new)
    }

    fn try_index(self) -> Option<usize> {
        self.get().try_index()
    }

    fn of(index: usize) -> Self {
        Self::new(I::of(index))
    }

    fn index(self) -> usize {
        self.get().index()
    }
}

impl<I: UntypedIndex> Index for EdgeIndex<I> {
    fn try_of(index: usize) -> Option<Self> {
        I::try_of(index).map(Self::new)
    }

    fn try_index(self) -> Option<usize> {
        self.get().try_index()
    }

    fn of(index: usize) -> Self {
        Self::new(I::of(index))
    }

    fn index(self) -> usize {
        self.get().index()
    }
}

impl<I: UntypedIndex> NodeIndex<I> {
    pub const fn new(inner: I) -> Self {
        Self { inner }
    }

    pub const fn get(self) -> I {
        self.inner
    }
}

impl<I: UntypedIndex> EdgeIndex<I> {
    pub const fn new(inner: I) -> Self {
        Self { inner }
    }

    pub const fn get(self) -> I {
        self.inner
    }
}

pub type DefaultNodeIndex = NodeIndex<DefaultUntypedIndex>;
pub type DefaultEdgeIndex = EdgeIndex<DefaultUntypedIndex>;

pub const fn node_index<I: UntypedIndex>(inner: I) -> NodeIndex<I> {
    NodeIndex::new(inner)
}

pub const fn edge_index<I: UntypedIndex>(inner: I) -> EdgeIndex<I> {
    EdgeIndex::new(inner)
}

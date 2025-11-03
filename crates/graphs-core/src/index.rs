use core::fmt;

use crate::{
    id::{EdgeType, EdgeTypeId, Id, NodeType, NodeTypeId, UntypedId},
    limit::Limited,
};

pub const OF: &str = "can not convert index to key";
pub const INDEX: &str = "can not convert key to index";

pub trait Index: Id {
    fn try_of(index: usize) -> Option<Self>;
    fn try_index(self) -> Option<usize>;

    #[must_use]
    fn of(index: usize) -> Self {
        Self::try_of(index).expect(OF)
    }

    fn index(self) -> usize {
        self.try_index().expect(INDEX)
    }
}

pub trait UntypedIndex: Index + UntypedId {}
pub trait NodeTypeIndex: Index + NodeTypeId {}
pub trait EdgeTypeIndex: Index + EdgeTypeId {}

impl<I: Index + UntypedId> UntypedIndex for I {}
impl<N: Index + NodeTypeId> NodeTypeIndex for N {}
impl<E: Index + EdgeTypeId> EdgeTypeIndex for E {}

macro_rules! impl_untyped_index {
    ($($int: ty),* $(,)?) => {
        $(
            impl $crate::index::Index for $int {
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

impl_untyped_index!(u8, u16, u32, u64, u128, usize);

pub type DefaultUntypedIndex = usize;

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

impl<I: UntypedIndex + fmt::Display> fmt::Display for NodeIndex<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<I: UntypedIndex + fmt::Display> fmt::Display for EdgeIndex<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<I: UntypedIndex> Limited for NodeIndex<I> {
    const LIMIT: Self = Self::new(I::LIMIT);

    fn is_limit(&self) -> bool {
        self.get().is_limit()
    }
}

impl<I: UntypedIndex> Limited for EdgeIndex<I> {
    const LIMIT: Self = Self::new(I::LIMIT);

    fn is_limit(&self) -> bool {
        self.get().is_limit()
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

pub const fn node_key<I: UntypedIndex>(inner: I) -> NodeIndex<I> {
    NodeIndex::new(inner)
}

pub const fn edge_key<I: UntypedIndex>(inner: I) -> EdgeIndex<I> {
    EdgeIndex::new(inner)
}

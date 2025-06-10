//! Indices of nodes and edges in simple graphs.

use core::fmt;

use graphs_core::{
    connection::Connection,
    id::{DefaultId, Id},
};

/// Represents node indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct NodeIndex<I: Id = DefaultId> {
    id: I,
}

impl<I: Id + fmt::Display> fmt::Display for NodeIndex<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<I: Id> Id for NodeIndex<I> {
    const LIMIT: Self = Self::new(I::LIMIT);

    fn of(index: usize) -> Self {
        Self::new(I::of(index))
    }

    fn index(self) -> usize {
        self.get().index()
    }
}

impl<I: Id> NodeIndex<I> {
    /// Constructs [`Self`] with the given identifier.
    pub const fn new(id: I) -> Self {
        Self { id }
    }

    /// Returns the contained identifier.
    pub const fn get(self) -> I {
        self.id
    }

    /// Switches this [`NodeIndex`] to [`EdgeIndex`].
    pub const fn switch(self) -> EdgeIndex<I> {
        EdgeIndex::new(self.id)
    }
}

/// Represents edge indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct EdgeIndex<I: Id = DefaultId> {
    id: I,
}

impl<I: Id + fmt::Display> fmt::Display for EdgeIndex<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<I: Id> Id for EdgeIndex<I> {
    const LIMIT: Self = Self::new(I::LIMIT);

    fn of(index: usize) -> Self {
        Self::new(I::of(index))
    }

    fn index(self) -> usize {
        self.get().index()
    }
}

impl<I: Id> EdgeIndex<I> {
    /// Constructs [`Self`] with the given identifier.
    pub const fn new(id: I) -> Self {
        Self { id }
    }

    /// Returns the contained identifier.
    pub const fn get(self) -> I {
        self.id
    }

    /// Switches this [`EdgeIndex`] to [`NodeIndex`].
    pub const fn switch(self) -> NodeIndex<I> {
        NodeIndex::new(self.id)
    }
}

/// Shorthand function used to create edge indices.
pub const fn edge_index<I: Id>(id: I) -> EdgeIndex<I> {
    EdgeIndex::new(id)
}

/// Shorthand function used to create node indices.
pub const fn node_index<I: Id>(id: I) -> NodeIndex<I> {
    NodeIndex::new(id)
}

/// Represents connections of indices.
pub type Connect<I = DefaultId> = Connection<NodeIndex<I>>;

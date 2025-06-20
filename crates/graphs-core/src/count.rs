//! Traits for graphs that are aware of their node and edge counts.

/// Represents graphs that are aware of their node count.
pub trait NodeCount {
    /// Returns the number of nodes in this graph.
    fn node_count(&self) -> usize;
}

impl<G: NodeCount + ?Sized> NodeCount for &G {
    fn node_count(&self) -> usize {
        (*self).node_count()
    }
}

impl<G: NodeCount + ?Sized> NodeCount for &mut G {
    fn node_count(&self) -> usize {
        (**self).node_count()
    }
}

/// Represents graphs that are aware of their edge count.
pub trait EdgeCount {
    /// Returns the number of edges in this graph.
    fn edge_count(&self) -> usize;
}

impl<G: EdgeCount + ?Sized> EdgeCount for &G {
    fn edge_count(&self) -> usize {
        (*self).edge_count()
    }
}

impl<G: EdgeCount + ?Sized> EdgeCount for &mut G {
    fn edge_count(&self) -> usize {
        (**self).edge_count()
    }
}

/// Combines node and edge counts together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Counts {
    /// The number of nodes.
    pub nodes: usize,

    /// The number of edges.
    pub edges: usize,
}

impl Counts {
    /// Constructs [`Self`].
    pub const fn new(nodes: usize, edges: usize) -> Self {
        Self { nodes, edges }
    }
}

/// Represents graphs that can provide both their node and edge counts.
///
/// This trait is automatically implemented for any type that implements
/// both [`NodeCount`] and [`EdgeCount`].
pub trait Count: NodeCount + EdgeCount {
    /// Returns the [`Counts`] of this graph.
    fn count(&self) -> Counts {
        Counts::new(self.node_count(), self.edge_count())
    }
}

impl<G: NodeCount + EdgeCount + ?Sized> Count for G {}

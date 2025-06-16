//! Traits for graphs that can provide capacities.

/// Represents graphs that can provide their node capacity.
pub trait NodeCapacity {
    /// Returns the node capacity of this graph.
    fn node_capacity(&self) -> usize;
}

impl<G: NodeCapacity + ?Sized> NodeCapacity for &G {
    fn node_capacity(&self) -> usize {
        (*self).node_capacity()
    }
}

impl<G: NodeCapacity + ?Sized> NodeCapacity for &mut G {
    fn node_capacity(&self) -> usize {
        (**self).node_capacity()
    }
}

/// Represents graphs that can provide their edge capacity.
pub trait EdgeCapacity {
    /// Returns the edge capacity of this graph.
    fn edge_capacity(&self) -> usize;
}

impl<G: EdgeCapacity + ?Sized> EdgeCapacity for &G {
    fn edge_capacity(&self) -> usize {
        (*self).edge_capacity()
    }
}

impl<G: EdgeCapacity + ?Sized> EdgeCapacity for &mut G {
    fn edge_capacity(&self) -> usize {
        (**self).edge_capacity()
    }
}

/// Combines node and edge capacities together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Capacities {
    /// The node capacity.
    pub nodes: usize,

    /// The edge capacity.
    pub edges: usize,
}

impl Capacities {
    /// Constructs [`Self`].
    pub const fn new(nodes: usize, edges: usize) -> Self {
        Self { nodes, edges }
    }
}

/// Represents graphs that can provide both their node and edge capacities.
///
/// This trait is automatically implemented for any type that implements
/// both [`NodeCapacity`] and [`EdgeCapacity`].
pub trait Capacity: NodeCapacity + EdgeCapacity {
    /// Returns the [`Capacities`] of this graph.
    fn capacity(&self) -> Capacities {
        Capacities::new(self.node_capacity(), self.edge_capacity())
    }
}

impl<G: NodeCapacity + EdgeCapacity + ?Sized> Capacity for G {}

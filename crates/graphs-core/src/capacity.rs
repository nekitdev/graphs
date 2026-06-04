//! Traits for graphs that can provide capacities.

use core::mem::replace;

use trait_aliases::trait_aliases;

use crate::base::Base;

/// Represents graphs that can provide their node capacity.
pub trait NodeCapacity: Base {
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
pub trait EdgeCapacity: Base {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Capacities {
    /// The node capacity.
    pub nodes: usize,

    /// The edge capacity.
    pub edges: usize,
}

impl Default for Capacities {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Capacities {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(nodes: usize, edges: usize) -> Self {
        Self { nodes, edges }
    }

    pub const fn reset(&mut self) -> Self {
        replace(self, Self::EMPTY)
    }

    pub const fn get(self) -> (usize, usize) {
        (self.nodes, self.edges)
    }

    pub const fn is_empty(&self) -> bool {
        self.nodes == 0 && self.edges == 0
    }

    pub const EMPTY: Self = Self::new(0, 0);
}

trait_aliases! {
    /// Represents graphs that can provide both their node and edge capacities.
    #[trait_alias(G)]
    pub trait Capacity = NodeCapacity + EdgeCapacity;
}

/// Implements the [`capacity`] method to provide [`Capacities`].
///
/// This trait combines [`node_capacity`] and [`edge_capacity`] together.
///
/// [`capacity`]: Self::capacity
/// [`node_capacity`]: NodeCapacity::node_capacity
/// [`edge_capacity`]: EdgeCapacity::edge_capacity
pub trait CapacityMethod: Capacity {
    /// Returns the [`Capacities`] of this graph.
    fn capacity(&self) -> Capacities {
        Capacities::new(self.node_capacity(), self.edge_capacity())
    }
}

impl<G: Capacity + ?Sized> CapacityMethod for G {}

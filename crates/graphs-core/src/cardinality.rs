//! Traits for graphs that are aware of their node and edge counts.

use core::mem::replace;

use trait_aliases::trait_aliases;

use crate::base::Base;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cardinality {
    pub order: usize,
    pub size: usize,
}

impl Default for Cardinality {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Cardinality {
    pub const fn new(order: usize, size: usize) -> Self {
        Self { order, size }
    }

    pub const fn reset(&mut self) -> Self {
        replace(self, Self::ZERO)
    }

    pub const fn get(self) -> (usize, usize) {
        (self.order, self.size)
    }

    pub const fn is_zero(&self) -> bool {
        self.order == 0 && self.size == 0
    }

    pub const ZERO: Self = Self::new(0, 0);
}

/// Represents graphs that are aware of their *order* (node count).
pub trait Order: Base {
    /// Returns the number of nodes in this graph.
    fn order(&self) -> usize;
}

impl<G: Order + ?Sized> Order for &G {
    fn order(&self) -> usize {
        (*self).order()
    }
}

impl<G: Order + ?Sized> Order for &mut G {
    fn order(&self) -> usize {
        (**self).order()
    }
}

/// Represents graphs that are aware of their *size* (edge count).
pub trait Size: Base {
    /// Returns the number of edges in this graph.
    fn size(&self) -> usize;
}

impl<G: Size + ?Sized> Size for &G {
    fn size(&self) -> usize {
        (*self).size()
    }
}

impl<G: Size + ?Sized> Size for &mut G {
    fn size(&self) -> usize {
        (**self).size()
    }
}

trait_aliases! {
    /// Represents graphs that can provide both their *order* and *size*.
    pub trait Count = Order + Size;
}

/// Implements additional methods for any [`Count`] graph.
pub trait CountMethods: Count {
    /// Returns the [`Cardinality`] of this graph.
    fn cardinality(&self) -> Cardinality {
        Cardinality::new(self.order(), self.size())
    }

    /// Checks whether the graph has no nodes and no edges, meaning it is the *null* graph.
    fn is_null(&self) -> bool {
        self.cardinality().is_zero()
    }
}

impl<G: Count + ?Sized> CountMethods for G {}

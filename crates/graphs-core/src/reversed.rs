//! Reversing directions in graphs.

use crate::{base::Base, visit::Visitable};

/// Represents wrappers that reverse directions in graphs.
pub struct Reversed<G> {
    graph: G,
}

impl<G> Reversed<G> {
    /// Constructs [`Self`] from the given graph.
    pub const fn new(graph: G) -> Self {
        Self { graph }
    }

    /// Returns the underlying graph behind immutable reference.
    pub const fn get(&self) -> &G {
        &self.graph
    }

    /// Returns the underlying graph behind mutable reference.
    pub const fn get_mut(&mut self) -> &mut G {
        &mut self.graph
    }

    /// Consumes [`Self`] and returns the contained graph.
    pub fn take(self) -> G {
        self.graph
    }
}

impl<G: Base> Base for Reversed<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<G: Visitable> Visitable for Reversed<G> {
    type Visitor = G::Visitor;

    fn build_visitor(&self) -> Self::Visitor {
        self.graph.build_visitor()
    }

    fn reset_visitor(&self, visitor: &mut Self::Visitor) {
        self.graph.reset_visitor(visitor);
    }
}

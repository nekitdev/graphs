//! Graph visiting traits and visitors.

use cfg_if::cfg_if;

use crate::{
    base::Base,
    id::{DefaultNodeId, NodeTypeId},
};

/// Represents visitors that can traverse graphs.
///
/// Visitors are generic over the node [`Id`] type that they can work with.
pub trait Visitor<N: NodeTypeId = DefaultNodeId> {
    /// Visits the node with the given ID.
    ///
    /// Returns [`false`] if the node was visited previously, otherwise returns [`true`].
    fn visit(&mut self, node: N) -> bool;

    /// Checks if the node with the given ID was visited previously.
    fn was_visited(&self, node: N) -> bool;

    /// Unvisits the node with the given ID.
    ///
    /// Returns [`true`] if the node was visited previously, otherwise returns [`false`].
    fn unvisit(&mut self, node: N) -> bool;
}

/// Represents graphs that can be visited.
pub trait Visit: Base {
    /// The associated type for the visitor that can traverse this graph.
    type Visitor: Visitor<Self::NodeId>;

    /// Builds and returns visitors for this graph.
    fn build_visitor(&self) -> Self::Visitor;

    /// Resets the given visitor.
    ///
    /// Implementations of this method can resize the visitor if necessary.
    fn reset_visitor(&self, visitor: &mut Self::Visitor);
}

impl<G: Visit + ?Sized> Visit for &G {
    type Visitor = G::Visitor;

    fn build_visitor(&self) -> Self::Visitor {
        (*self).build_visitor()
    }

    fn reset_visitor(&self, visitor: &mut Self::Visitor) {
        (*self).reset_visitor(visitor);
    }
}

impl<G: Visit + ?Sized> Visit for &mut G {
    type Visitor = G::Visitor;

    fn build_visitor(&self) -> Self::Visitor {
        (**self).build_visitor()
    }

    fn reset_visitor(&self, visitor: &mut Self::Visitor) {
        (**self).reset_visitor(visitor);
    }
}

cfg_if! {
    if #[cfg(feature = "std")] {
        use core::hash::BuildHasher;

        use std::collections::HashSet;

        impl<N: NodeTypeId, S: BuildHasher> Visitor<N> for HashSet<N, S> {
            fn visit(&mut self, node: N) -> bool {
                self.insert(node)
            }

            fn was_visited(&self, node: N) -> bool {
                self.contains(&node)
            }

            fn unvisit(&mut self, node: N) -> bool {
                self.remove(&node)
            }
        }
    }
}

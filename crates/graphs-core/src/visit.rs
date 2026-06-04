//! Graph visiting traits and visitors.

use crate::{base::Base, id::NodeTypeId};

/// Represents the output as returned by [`visit`] and [`unvisit`].
///
/// [`visit`]: Visitor::visit
/// [`unvisit`]: Visitor::unvisit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Output {
    /// The node was *previously* (un)visited.
    Previously,

    /// The node was *newly* (un)visited.
    Newly,
}

impl Output {
    /// Checks whether the output is [`Previously`].
    ///
    /// [`Previously`]: Self::Previously
    pub const fn is_previously(self) -> bool {
        matches!(self, Self::Previously)
    }

    /// Checks whether the output is [`Newly`].
    ///
    /// [`Newly`]: Self::Newly
    pub const fn is_newly(self) -> bool {
        matches!(self, Self::Newly)
    }
}

/// Represents visitors that can traverse graphs.
///
/// Visitors are generic over the [`NodeTypeId`] type that they can work with.
pub trait Visitor<N: NodeTypeId> {
    /// Visits the node with the given ID.
    fn visit(&mut self, node: N) -> Output;

    /// Checks if the node with the given ID was visited previously.
    fn was_visited(&self, node: N) -> bool;

    /// Unvisits the node with the given ID.
    fn unvisit(&mut self, node: N) -> Output;
}

impl<N: NodeTypeId, V: Visitor<N>> Visitor<N> for &mut V {
    fn visit(&mut self, node: N) -> Output {
        (*self).visit(node)
    }

    fn was_visited(&self, node: N) -> bool {
        (**self).was_visited(node)
    }

    fn unvisit(&mut self, node: N) -> Output {
        (*self).unvisit(node)
    }
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

#[cfg(feature = "std")]
mod hash {
    use core::hash::BuildHasher;

    use std::collections::HashSet;

    use crate::id::NodeTypeId;

    use super::Visitor;

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

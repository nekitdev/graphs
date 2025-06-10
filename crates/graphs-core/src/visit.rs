//! Graph visiting traits and visitors.

use cfg_if::cfg_if;

use crate::{base::Base, id::Id};

/// Represents visitors that can traverse graphs.
///
/// Visitors are generic over the [`Id`] type that they can work with.
pub trait Visitor<I: Id> {
    /// Visits the node with the given ID.
    ///
    /// Returns [`false`] if the node was visited previously, otherwise returns [`true`].
    fn visit(&mut self, id: I) -> bool;

    /// Checks if the node with the given ID was visited previously.
    fn was_visited(&self, id: I) -> bool;

    /// Unvisits the node with the given ID.
    ///
    /// Returns [`true`] if the node was visited previously, otherwise returns [`false`].
    fn unvisit(&mut self, id: I) -> bool;
}

/// Represents graphs that can be visited.
pub trait Visitable: Base {
    /// The associated type for the visitor that can traverse this graph.
    type Visitor: Visitor<Self::NodeId>;

    /// Builds and returns visitors for this graph.
    fn build_visitor(&self) -> Self::Visitor;

    /// Resets the given visitor.
    ///
    /// Implementations of this method can resize the visitor if necessary.
    fn reset_visitor(&self, visitor: &mut Self::Visitor);
}

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::{collections::HashSet, hash::BuildHasher};

        impl<I: Id, S: BuildHasher> Visitor<I> for HashSet<I, S> {
            fn visit(&mut self, id: I) -> bool {
                self.insert(id)
            }

            fn was_visited(&self, id: I) -> bool {
                self.contains(&id)
            }

            fn unvisit(&mut self, id: I) -> bool {
                self.remove(&id)
            }
        }
    }
}

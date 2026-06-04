use core::fmt;

use graphs_core::{
    id::{DefaultUntypedId, EdgeType, Id, NodeType, UntypedId},
    sentinel::Sentinel,
};

/// Represents *node* identifiers wrapping some *untyped* [`Id`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NodeId<I: UntypedId = DefaultUntypedId> {
    untyped: I,
}

/// Represents *edge* identifiers wrapping some *untyped* [`Id`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EdgeId<I: UntypedId = DefaultUntypedId> {
    untyped: I,
}

#[cfg(feature = "serde")]
mod serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{EdgeId, NodeId, UntypedId};

    impl<I: UntypedId> Serialize for NodeId<I> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.get().serialize(serializer)
        }
    }

    impl<'de, I: UntypedId> Deserialize<'de> for NodeId<I> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let inner = I::deserialize(deserializer)?;

            let node_id = Self::new(inner);

            Ok(node_id)
        }
    }

    impl<I: UntypedId> Serialize for EdgeId<I> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.get().serialize(serializer)
        }
    }

    impl<'de, I: UntypedId> Deserialize<'de> for EdgeId<I> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let inner = I::deserialize(deserializer)?;

            let edge_id = Self::new(inner);

            Ok(edge_id)
        }
    }
}

impl<I: UntypedId> fmt::Display for NodeId<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{untyped}", untyped = self.get())
    }
}

impl<I: UntypedId> fmt::Display for EdgeId<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{untyped}", untyped = self.get())
    }
}

impl<I: UntypedId> NodeId<I> {
    /// Constructs [`Self`].
    pub const fn new(untyped: I) -> Self {
        Self { untyped }
    }

    /// Returns the contained *untyped* identifier.
    pub const fn get(self) -> I {
        self.untyped
    }
}

impl<I: UntypedId> EdgeId<I> {
    /// Constructs [`Self`].
    pub const fn new(untyped: I) -> Self {
        Self { untyped }
    }

    /// Returns the contained *untyped* identifier.
    pub const fn get(self) -> I {
        self.untyped
    }
}

impl<I: UntypedId> Sentinel for NodeId<I> {
    const SENTINEL: Self = Self::new(I::SENTINEL);

    fn is_sentinel(&self) -> bool {
        self.get().is_sentinel()
    }
}

impl<I: UntypedId> Sentinel for EdgeId<I> {
    const SENTINEL: Self = Self::new(I::SENTINEL);

    fn is_sentinel(&self) -> bool {
        self.get().is_sentinel()
    }
}

impl<I: UntypedId> Id for NodeId<I> {
    type Type = NodeType;
}

impl<I: UntypedId> Id for EdgeId<I> {
    type Type = EdgeType;
}

/// The default *node* [`Id`].
pub type DefaultNodeId = NodeId<DefaultUntypedId>;

/// The default *edge* [`Id`].
pub type DefaultEdgeId = EdgeId<DefaultUntypedId>;

/// Shorthand for calling [`NodeId::new`].
pub const fn node_id<I: UntypedId>(inner: I) -> NodeId<I> {
    NodeId::new(inner)
}

/// Shorthand for calling [`EdgeId::new`].
pub const fn edge_id<I: UntypedId>(inner: I) -> EdgeId<I> {
    EdgeId::new(inner)
}

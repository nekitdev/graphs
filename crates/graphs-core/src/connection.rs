//! Connections between nodes in graphs.

use core::{fmt, mem::swap};

use crate::{
    direction::{Direction, Incoming, Outgoing},
    id::{DefaultNodeId, EdgeType, Id, NodeTypeId},
    kind::Kind,
};

/// Represents connections between nodes in graphs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Connection<N: NodeTypeId = DefaultNodeId> {
    /// The source node identifier.
    pub source: N,

    /// The target node identifier.
    pub target: N,
}

impl<N: NodeTypeId + fmt::Display> fmt::Display for Connection<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{source} -> {target}",
            source = self.source,
            target = self.target
        )
    }
}

impl<N: NodeTypeId> Connection<N> {
    /// Constructs [`Self`] from the given source and target identifiers.
    pub const fn new(source: N, target: N) -> Self {
        Self { source, target }
    }

    /// Reverses the connection, swapping the source and target identifiers.
    pub const fn reverse(&mut self) {
        swap(&mut self.source, &mut self.target);
    }

    /// Copies the connection.
    pub const fn copy(&self) -> Self {
        Self::new(self.source, self.target)
    }

    /// Returns the connection based on the specified direction.
    pub const fn directed(&self, direction: Direction) -> N {
        match direction {
            Outgoing => self.source,
            Incoming => self.target,
        }
    }

    pub const fn directed_canonical(&self) -> Canonical<N> {
        Canonical::new(self.source, self.target)
    }

    pub fn undirected_canonical(&self) -> Canonical<N> {
        let target = self.target;
        let source = self.source;

        if target < source {
            Canonical::new(target, source)
        } else {
            Canonical::new(source, target)
        }
    }

    pub fn canonical<K: Kind>(&self) -> Canonical<N> {
        if K::IS_DIRECTED {
            self.directed_canonical()
        } else {
            self.undirected_canonical()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Canonical<N: NodeTypeId = DefaultNodeId> {
    source: N,
    target: N,
}

impl<N: NodeTypeId> Canonical<N> {
    const fn new(source: N, target: N) -> Self {
        Self { source, target }
    }

    pub const fn get(self) -> (N, N) {
        (self.source, self.target)
    }
}

impl<N: NodeTypeId> Id for Canonical<N> {
    type Type = EdgeType;

    const LIMIT: Self = Self::new(N::LIMIT, N::LIMIT);
}

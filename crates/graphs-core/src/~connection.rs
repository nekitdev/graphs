//! Connections between nodes in graphs.

use core::{fmt, mem::swap};

use crate::{
    base::Base,
    canonical::{Canonicalize, Canonicalized},
    direction::{Direction, Incoming, Outgoing},
    id::{DefaultNodeId, EdgeType, Id, NodeTypeId},
    keys::{DefaultNodeIndex, NodeTypeIndex},
};

/// Represents connections between nodes in graphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Connection<N: NodeTypeId = DefaultNodeId> {
    /// The source node identifier.
    pub source: N,

    /// The target node identifier.
    pub target: N,
}

pub type DefaultConnection = Connection<DefaultNodeId>;

impl<N: NodeTypeId> From<Connection<N>> for (N, N) {
    fn from(connection: Connection<N>) -> Self {
        connection.parts()
    }
}

impl<N: NodeTypeId> From<(N, N)> for Connection<N> {
    fn from((source, target): (N, N)) -> Self {
        Self::new(source, target)
    }
}

impl<N: NodeTypeId> Default for Connection<N> {
    fn default() -> Self {
        Self::limit()
    }
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

impl<N: NodeTypeId> Id for Connection<N> {
    type Type = EdgeType;

    const LIMIT: Self = Self::limit();
}

impl<N: NodeTypeId> Connection<N> {
    /// Constructs [`Self`] from the given source and target identifiers.
    pub const fn new(source: N, target: N) -> Self {
        Self { source, target }
    }

    pub const fn limit() -> Self {
        Self::new(N::LIMIT, N::LIMIT)
    }

    /// Reverses the connection, swapping the source and target identifiers.
    pub const fn reverse(&mut self) {
        swap(&mut self.source, &mut self.target);
    }

    /// Returns the connection based on the specified direction.
    pub const fn directed(&self, direction: Direction) -> N {
        match direction {
            Outgoing => self.source,
            Incoming => self.target,
        }
    }

    pub const fn parts(&self) -> (N, N) {
        (self.source, self.target)
    }

    pub fn sort(&mut self) {
        if self.target < self.source {
            self.reverse();
        }
    }

    pub fn is_loop(&self) -> bool {
        self.source == self.target
    }
}

pub const fn connection<N: NodeTypeId>(source: N, target: N) -> Connection<N> {
    Connection::new(source, target)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexConnection<N: NodeTypeIndex = DefaultNodeIndex> {
    pub source: N,
    pub target: N,
}

impl<N: NodeTypeIndex> From<Connection<N>> for IndexConnection<N> {
    fn from(connection: Connection<N>) -> Self {
        Self::new(connection.source, connection.target)
    }
}

impl<N: NodeTypeIndex> From<IndexConnection<N>> for Connection<N> {
    fn from(connection: IndexConnection<N>) -> Self {
        Self::new(connection.source, connection.target)
    }
}

impl<N: NodeTypeIndex> From<IndexConnection<N>> for (N, N) {
    fn from(connection: IndexConnection<N>) -> Self {
        connection.parts()
    }
}

impl<N: NodeTypeIndex> From<(N, N)> for IndexConnection<N> {
    fn from((source, target): (N, N)) -> Self {
        Self::new(source, target)
    }
}

impl<N: NodeTypeIndex> Default for IndexConnection<N> {
    fn default() -> Self {
        Self::limit()
    }
}

impl<N: NodeTypeIndex + fmt::Display> fmt::Display for IndexConnection<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{source} -> {target}",
            source = self.source,
            target = self.target
        )
    }
}

impl<N: NodeTypeIndex> Id for IndexConnection<N> {
    type Type = EdgeType;

    const LIMIT: Self = Self::limit();
}

impl<N: NodeTypeIndex> IndexConnection<N> {
    /// Constructs [`Self`] from the given source and target indices.
    pub const fn new(source: N, target: N) -> Self {
        Self { source, target }
    }

    pub fn of(source: usize, target: usize) -> Self {
        Self::new(N::of(source), N::of(target))
    }

    pub const fn limit() -> Self {
        Self::new(N::LIMIT, N::LIMIT)
    }

    /// Reverses the connection, swapping the source and target indices.
    pub const fn reverse(&mut self) {
        swap(&mut self.source, &mut self.target);
    }

    /// Returns the connection based on the specified direction.
    pub const fn directed(&self, direction: Direction) -> N {
        match direction {
            Outgoing => self.source,
            Incoming => self.target,
        }
    }

    pub const fn parts(&self) -> (N, N) {
        (self.source, self.target)
    }

    pub fn sort(&mut self) {
        if self.target < self.source {
            self.reverse();
        }
    }

    pub fn is_loop(&self) -> bool {
        self.source == self.target
    }
}

pub const fn index_connection<N: NodeTypeIndex>(source: N, target: N) -> IndexConnection<N> {
    IndexConnection::new(source, target)
}

pub fn index_connection_of<N: NodeTypeIndex>(source: usize, target: usize) -> IndexConnection<N> {
    IndexConnection::of(source, target)
}

pub trait Connections: Base {
    type Iterator<'n>: Iterator<Item = Connection<Self::NodeId>>
    where
        Self: 'n;

    fn connections(&self) -> Self::Iterator<'_>;

    fn canonical(&self) -> Canonicalized<Self::Kind, Self::Iterator<'_>, Self::NodeId> {
        self.connections().canonicalize()
    }
}

//! Connections between nodes in graphs.

use core::{fmt, mem::swap};

use crate::{
    direction::{Direction, Incoming, Outgoing},
    id::{DefaultId, Id},
};

/// Represents connections between nodes in graphs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Connection<I: Id = DefaultId> {
    /// The source node identifier.
    pub source: I,

    /// The target node identifier.
    pub target: I,
}

impl<I: Id + fmt::Display> fmt::Display for Connection<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{source} -> {target}",
            source = self.source,
            target = self.target
        )
    }
}

impl<I: Id> Connection<I> {
    /// Constructs [`Self`] from the given source and target identifiers.
    pub const fn new(source: I, target: I) -> Self {
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
    pub const fn directed(&self, direction: Direction) -> I {
        match direction {
            Outgoing => self.source,
            Incoming => self.target,
        }
    }
}

/// Represents types that establish connections.
pub trait Connector {
    /// The associated type of the identifier.
    type Id: Id;

    /// Returns the connection.
    fn connection(&self) -> Connection<Self::Id>;
}

impl<C: Connector + ?Sized> Connector for &C {
    type Id = C::Id;

    fn connection(&self) -> Connection<Self::Id> {
        (*self).connection()
    }
}

impl<C: Connector + ?Sized> Connector for &mut C {
    type Id = C::Id;

    fn connection(&self) -> Connection<Self::Id> {
        (**self).connection()
    }
}

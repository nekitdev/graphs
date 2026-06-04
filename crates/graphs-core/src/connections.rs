//! Connections in graphs.

use trait_aliases::trait_aliases;

use crate::{
    direction::Direction,
    id::NodeTypeId,
    kinds::{Directed, Kind, Undirected},
};

/// Represents connections between nodes in graphs.
pub trait Connection: Copy {
    /// The associated type for node IDs.
    type NodeId: NodeTypeId;

    /// The associated type for the connection [`Kind`], either [`Directed`] or [`Undirected`].
    ///
    /// [`Directed`]: kinds::Directed
    /// [`Undirected`]: kinds::Undirected
    type Kind: Kind;

    /// The *inverse* type of this connection, which is any [`Connection`]
    /// with the same [`NodeId`] type, but [`Kind`] being the [`Inverse`].
    ///
    /// [`NodeId`]: Self::NodeId
    /// [`Kind`]: Self::Kind
    /// [`Inverse`]: Kind::Inverse
    type Inverse: Connection<NodeId = Self::NodeId, Kind = <Self::Kind as Kind>::Inverse>;

    /// Constructs [`Self`] connecting the provided nodes.
    fn connecting(one: Self::NodeId, two: Self::NodeId) -> Self;

    /// Returns the *parts* of the connection.
    fn parts(&self) -> Parts<Self>;

    /// Returns the *inverse* of this connection.
    fn inverse(self) -> Self::Inverse;

    fn directed(&self, direction: Direction) -> Self::NodeId {
        let (outgoing, incoming) = self.parts();

        match direction {
            Direction::Outgoing => outgoing,
            Direction::Incoming => incoming,
        }
    }

    /// Checks whether the connection is looped, that is, connects some node to itself.
    fn is_loop(&self) -> bool {
        let (one, two) = self.parts();

        one == two
    }
}

/// Represents the *parts* of the given [`Connection`] of type `C`.
///
/// This is simply the `(C::NodeId, C::NodeId)` tuple.
pub type Parts<C> = (<C as Connection>::NodeId, <C as Connection>::NodeId);

trait_aliases! {
    /// Represents *directed* connections.
    ///
    /// Implemented for any [`Connection`] with [`Kind`] set to [`Directed`].
    ///
    /// [`Kind`]: Connection::Kind
    #[trait_alias(C)]
    pub trait DirectedConnection = Connection<Kind = Directed>;

    /// Represents *undirected* connections.
    ///
    /// Implemented for any [`Connection`] with [`Kind`] set to [`Undirected`].
    ///
    /// [`Kind`]: Connection::Kind
    #[trait_alias(C)]
    pub trait UndirectedConnection = Connection<Kind = Undirected>;
}

/// Represents connections that can be reversed.
pub trait ReverseConnection: DirectedConnection {
    /// Reverses the direction of the connection.
    fn reverse(&mut self);
}

pub trait DirectedConnectionMethods: DirectedConnection {
    fn source(&self) -> Self::NodeId {
        let (source, _) = self.parts();

        source
    }

    fn target(&self) -> Self::NodeId {
        let (_, target) = self.parts();

        target
    }

    fn reversed(&self) -> Self {
        let (source, target) = self.parts();

        Self::connecting(target, source)
    }
}

pub trait UndirectedConnectionMethods: UndirectedConnection {
    fn one(&self) -> Self::NodeId {
        let (one, _) = self.parts();

        one
    }

    fn two(&self) -> Self::NodeId {
        let (_, two) = self.parts();

        two
    }
}

impl<C: DirectedConnection> DirectedConnectionMethods for C {}

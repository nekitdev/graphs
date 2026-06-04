//! Base traits for graphs.

use trait_aliases::trait_aliases;

use crate::{
    connections::Connection,
    id::{EdgeTypeId, NodeTypeId},
    kinds::{self, Kind},
    loops::{Allow, Forbid, Loop},
    types::{Multiple, Single, Type},
};

/// Represents the base definition of any graph.
///
/// There are several trait aliases, for various combinations
/// of the associated [`Loop`] and [`Type`] types.
///
/// | `Alias`    | [`Loop`]   | [`Type`]     |
/// |------------|------------|--------------|
/// | [`Simple`] | [`Forbid`] | [`Single`]   |
/// | [`Looped`] | [`Allow`]  | [`Single`]   |
/// | [`Multi`]  | [`Forbid`] | [`Multiple`] |
/// | [`Pseudo`] | [`Allow`]  | [`Multiple`] |
///
/// Additionally, there are [`ForbidLoop`] and [`AllowLoop`]
/// along with [`SingleType`] and [`MultipleType`], which allow restrictions on
/// only one of the associated types instead of their combination.
///
/// Finally, there are trait aliases based on the associated [`Kind`] types,
/// namely [`Directed`] and [`Undirected`].
pub trait Base {
    /// The associated type for node identifiers.
    type NodeId: NodeTypeId;

    /// The associated type for edge identifiers.
    type EdgeId: EdgeTypeId;

    /// The associated type for node connections.
    type Connection: Connection<NodeId = Self::NodeId, Kind = Self::Kind>;

    /// The associated type for graph kinds, either [`Directed`] or [`Undirected`].
    ///
    /// [`Directed`]: kinds::Directed
    /// [`Undirected`]: kinds::Undirected
    type Kind: Kind;

    /// The associated types for graph edges, either [`Single`] or [`Multiple`].
    type Type: Type;

    /// The associated types for graph loops, either [`Forbid`] or [`Allow`].
    type Loop: Loop;
}

impl<G: Base + ?Sized> Base for &G {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;

    type Connection = G::Connection;

    type Kind = G::Kind;
    type Type = G::Type;
    type Loop = G::Loop;
}

impl<G: Base + ?Sized> Base for &mut G {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;

    type Connection = G::Connection;

    type Kind = G::Kind;
    type Type = G::Type;
    type Loop = G::Loop;
}

trait_aliases! {
    /// Represents graphs that *forbid* loops.
    ///
    /// Implemented for any [`Base`] graph with [`Loop`] set to [`Forbid`]
    ///
    /// [`Loop`]: Base::Loop
    #[trait_alias(G)]
    pub trait ForbidLoop = Base<Loop = Forbid>;

    /// Represents graphs that *allow* loops.
    ///
    /// Implemented for any [`Base`] graph with [`Loop`] set to [`Allow`].
    ///
    /// [`Loop`]: Base::Loop
    #[trait_alias(G)]
    pub trait AllowLoop = Base<Loop = Allow>;

    /// Represents graphs that have *single* edges.
    ///
    /// Implemented for any [`Base`] graph with [`Type`] set to [`Single`].
    ///
    /// [`Type`]: Base::Type
    #[trait_alias(G)]
    pub trait SingleType = Base<Type = Single>;

    /// Represents graphs that have *multiple* edges.
    ///
    /// Implemented for any [`Base`] graph with [`Type`] set to [`Multiple`].
    ///
    /// [`Type`]: Base::Type
    #[trait_alias(G)]
    pub trait MultipleType = Base<Type = Multiple>;

    /// Represents graphs that *forbid* loops and have *single* edges.
    ///
    /// Implemented for any [`ForbidLoop`] and [`SingleType`] graph.
    #[trait_alias(G)]
    pub trait Simple = ForbidLoop + SingleType;

    /// Represents graphs that *allow* loops and have *single* edges.
    ///
    /// Implemented for any [`AllowLoop`] and [`SingleType`] graph.
    #[trait_alias(G)]
    pub trait Looped = AllowLoop + SingleType;

    /// Represents graphs that *forbid* loops and have *multiple* edges.
    ///
    /// Implemented for any [`ForbidLoop`] and [`MultipleType`] graph.
    #[trait_alias(G)]
    pub trait Multi = ForbidLoop + MultipleType;

    /// Represents graphs that *allow* loops and have *multiple* edges.
    ///
    /// Implemented for any [`AllowLoop`] and [`MultipleType`] graph.
    #[trait_alias(G)]
    pub trait Pseudo = AllowLoop + MultipleType;

    /// Represents *directed* graphs.
    ///
    /// Implemented for any [`Base`] graph with [`Kind`] set to [`Directed`].
    ///
    /// [`Kind`]: Base::Kind
    #[trait_alias(G)]
    pub trait Directed = Base<Kind = kinds::Directed>;

    /// Represents *undirected* graphs.
    ///
    /// Implemented for any [`Base`] graph with [`Kind`] set to [`Undirected`].
    ///
    /// [`Kind`]: Base::Kind
    #[trait_alias(G)]
    pub trait Undirected = Base<Kind = kinds::Undirected>;
}

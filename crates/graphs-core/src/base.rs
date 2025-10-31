//! Base traits for graphs.

use crate::{
    algorithm::Algorithm,
    connections::Connection,
    id::{EdgeTypeId, NodeTypeId},
    kinds::{self, Kind},
    loops::{Allow, Forbid, Loop},
    types::{Multiple, Single, Type},
};

/// Represents the base definition of any graph.
///
/// There are several trait aliases, for variations of the associated [`Loop`] and [`Type`] types.
///
/// | `Alias`    | [`Loop`]   | [`Type`]     |
/// |------------|------------|--------------|
/// | [`Simple`] | [`Forbid`] | [`Single`]   |
/// | [`Looped`] | [`Allow`]  | [`Single`]   |
/// | [`Multi`]  | [`Forbid`] | [`Multiple`] |
/// | [`Pseudo`] | [`Allow`]  | [`Multiple`] |
///
/// Additionally, there are trait aliases based on the associated [`Kind`] types,
/// namely [`Directed`] and [`Undirected`].
pub trait Base {
    /// The associated type for node identifiers.
    type NodeId: NodeTypeId;

    /// The associated type for edge identifiers.
    type EdgeId: EdgeTypeId;

    type Connection: Connection<Id = Self::NodeId, Kind = Self::Kind>;

    /// The associated type for graph [`Kind`], [`Directed`] or [`Undirected`].
    ///
    /// [`Directed`]: crate::kinds::Directed
    /// [`Undirected`]: crate::kinds::Undirected
    type Kind: Kind;

    /// The associated type for graph [`Type`], [`Single`] or [`Multiple`].
    type Type: Type;

    /// The associated type for graph [`Loop`], [`Allow`] or [`Forbid`].
    type Loop: Loop;

    /// Applies the given algorithm to the graph.
    ///
    /// Use [`by_ref`] or [`by_mut`] to apply the algorithm by reference or mutable reference,
    /// respectively, instead of taking ownership.
    ///
    /// [`by_ref`]: crate::by::By::by_ref
    /// [`by_mut`]: crate::by::By::by_mut
    fn apply<A: Algorithm<Self>>(self, mut algorithm: A) -> A::Output
    where
        Self: Sized,
    {
        algorithm.perform(self)
    }
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

/// Represents graphs that *forbid* loops and have *single* edges.
pub trait Simple: Base<Loop = Forbid, Type = Single> {}

/// Represents graphs that *allow* loops and otherwise have *single* edges.
pub trait Looped: Base<Loop = Allow, Type = Single> {}

/// Represents graphs that *forbid* loops and have *multiple* edges.
pub trait Multi: Base<Loop = Forbid, Type = Multiple> {}

/// Represents graphs that *allow* loops and have *multiple* edges.
pub trait Pseudo: Base<Loop = Allow, Type = Multiple> {}

impl<G: Base<Loop = Forbid, Type = Single> + ?Sized> Simple for G {}
impl<G: Base<Loop = Allow, Type = Single> + ?Sized> Looped for G {}
impl<G: Base<Loop = Forbid, Type = Multiple> + ?Sized> Multi for G {}
impl<G: Base<Loop = Allow, Type = Multiple> + ?Sized> Pseudo for G {}

pub const fn assert_simple<G: Simple + ?Sized>() {}
pub const fn assert_looped<G: Looped + ?Sized>() {}
pub const fn assert_multi<G: Multi + ?Sized>() {}
pub const fn assert_pseudo<G: Pseudo + ?Sized>() {}

/// Represents *directed* graphs.
pub trait Directed: Base<Kind = kinds::Directed> {}

/// Represents *undirected* graphs.
pub trait Undirected: Base<Kind = kinds::Undirected> {}

impl<G: Base<Kind = kinds::Directed> + ?Sized> Directed for G {}
impl<G: Base<Kind = kinds::Undirected> + ?Sized> Undirected for G {}

pub const fn assert_directed<G: Directed + ?Sized>() {}
pub const fn assert_undirected<G: Undirected + ?Sized>() {}

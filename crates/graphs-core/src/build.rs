//! Building graphs.

use crate::{base::Base, connections::Connection, data::Data, recoverable::RecoverableResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge<T, C: Connection> {
    pub connection: C,
    pub value: T,
}

pub type EdgeOf<G> = Edge<<G as Data>::EdgeValue, <G as Base>::Connection>;

impl<T, C: Connection> Edge<T, C> {
    pub const fn new(connection: C, value: T) -> Self {
        Self { connection, value }
    }
}

impl<T: Default, C: Connection> Edge<T, C> {
    pub fn new_default(connection: C) -> Self {
        Self::new(connection, T::default())
    }
}

pub type AddNodeResult<G> =
    RecoverableResult<<G as Base>::NodeId, <G as Build>::NodeError, <G as Data>::NodeValue>;

pub type AddEdgeResult<G> =
    RecoverableResult<<G as Base>::EdgeId, <G as Build>::EdgeError, <G as Data>::EdgeValue>;

/// Represents graphs that can be built incrementally.
pub trait Build: Data {
    /// The associated type for errors that can occur when adding nodes.
    type NodeError;

    /// The associated type for errors that can occur when adding edges.
    type EdgeError;

    /// Attempts to add new node with the given value to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`NodeError`] if the node could not be added.
    ///
    /// [`NodeError`]: Self::NodeError
    fn add_node(&mut self, value: Self::NodeValue) -> AddNodeResult<Self>;

    /// Attempts to add new [`Edge`].
    ///
    /// # Errors
    ///
    /// Returns [`EdgeError`] if the edge could not be added.
    ///
    /// [`EdgeError`]: Self::EdgeError
    fn add_edge(&mut self, edge: EdgeOf<Self>) -> AddEdgeResult<Self>;

    fn add_edge_with(
        &mut self,
        connection: Self::Connection,
        value: Self::EdgeValue,
    ) -> AddEdgeResult<Self> {
        self.add_edge(Edge::new(connection, value))
    }

    fn add_edge_connecting(
        &mut self,
        one: Self::NodeId,
        two: Self::NodeId,
        value: Self::EdgeValue,
    ) -> AddEdgeResult<Self> {
        self.add_edge_with(Self::Connection::connecting(one, two), value)
    }
}

impl<G: Build + ?Sized> Build for &mut G {
    type NodeError = G::NodeError;
    type EdgeError = G::EdgeError;

    fn add_node(&mut self, value: Self::NodeValue) -> AddNodeResult<Self> {
        (*self).add_node(value)
    }

    fn add_edge(&mut self, edge: EdgeOf<Self>) -> AddEdgeResult<Self> {
        (*self).add_edge(edge)
    }
}

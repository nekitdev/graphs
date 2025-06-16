//! Building graphs.

use core::error::Error;

use crate::{
    connection::Connection,
    data::Data,
    id::{DefaultEdgeId, EdgeTypeId},
    internal::failed,
};

pub struct Output<T, E: EdgeTypeId = DefaultEdgeId> {
    pub edge: E,
    pub previous: Option<T>,
}

impl<T, E: EdgeTypeId> Output<T, E> {
    pub const fn new(edge: E, previous: Option<T>) -> Self {
        Self { edge, previous }
    }

    pub const fn not_taken(edge: E) -> Self {
        Self::new(edge, None)
    }

    pub const fn taken(edge: E, value: T) -> Self {
        Self::new(edge, Some(value))
    }
}

/// Represents graphs that can be built incrementally.
pub trait Build: Data {
    /// The associated type for errors that can occur when adding nodes.
    type AddNodeError: Error;

    /// The associated type for errors that can occur when adding edges.
    type AddEdgeError: Error;

    /// Attempts to add new node with the given value to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`AddNodeError`] if the node could not be added.
    ///
    /// Note to implementors: the node value should ideally be recoverable in error case.
    ///
    /// [`AddNodeError`]: Self::AddNodeError
    fn try_add_node(&mut self, value: Self::NodeValue) -> Result<Self::NodeId, Self::AddNodeError>;

    /// Attempts to add new edge with the given connection and the specified value.
    ///
    /// # Errors
    ///
    /// Returns [`AddEdgeError`] if the edge could not be added.
    ///
    /// Note to implementors: the edge value should ideally be recoverable in error case.
    ///
    /// [`AddEdgeError`]: Self::AddEdgeError
    fn try_add_edge_with(
        &mut self,
        connection: Connection<Self::NodeId>,
        value: Self::EdgeValue,
    ) -> Result<Output<Self::EdgeValue, Self::EdgeId>, Self::AddEdgeError>;

    fn try_add_edge(
        &mut self,
        source: Self::NodeId,
        target: Self::NodeId,
        value: Self::EdgeValue,
    ) -> Result<Output<Self::EdgeValue, Self::EdgeId>, Self::AddEdgeError> {
        self.try_add_edge_with(Connection::new(source, target), value)
    }

    /// Panicking version of [`try_add_node`].
    ///
    /// # Panics
    ///
    /// Panics if the node could not be added.
    ///
    /// [`try_add_node`]: Self::try_add_node
    fn add_node(&mut self, value: Self::NodeValue) -> Self::NodeId {
        self.try_add_node(value).expect(failed!(add_node))
    }

    /// Panicking version of [`try_add_edge_with`].
    ///
    /// # Panics
    ///
    /// Panics if the edge could not be added.
    ///
    /// [`try_add_edge_with`]: Self::try_add_edge_with
    fn add_edge_with(
        &mut self,
        connection: Connection<Self::NodeId>,
        value: Self::EdgeValue,
    ) -> Output<Self::EdgeValue, Self::EdgeId> {
        self.try_add_edge_with(connection, value)
            .expect(failed!(add_edge_with))
    }

    fn add_edge(
        &mut self,
        source: Self::NodeId,
        target: Self::NodeId,
        value: Self::EdgeValue,
    ) -> Output<Self::EdgeValue, Self::EdgeId> {
        self.try_add_edge(source, target, value)
            .expect(failed!(add_edge))
    }
}

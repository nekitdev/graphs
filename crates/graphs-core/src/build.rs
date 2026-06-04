//! Building graphs.

use core::error::Error;

use trait_aliases::trait_aliases;

use crate::{
    base::Base,
    data::Data,
    edges::{Edge, EdgeIn},
    nodes::{Node, NodeIn},
    recoverable::Recoverable,
};

/// The result type for adding nodes to graphs, which can be used to extract node
/// values in case of errors, allowing for better recovery.
pub type RecoverableNode<G> =
    Recoverable<<G as Base>::NodeId, <G as TryAddNodes>::Error, <G as Data>::NodeValue>;

/// The result type for adding edges to graphs, which can be used to extract edge
/// values in case of errors, allowing for better recovery.
pub type RecoverableEdge<G> =
    Recoverable<<G as Base>::EdgeId, <G as TryAddEdges>::Error, <G as Data>::EdgeValue>;

/// Represents graphs that can have nodes added to them.
///
/// All methods in this trait return [`Recoverable`] results, which can be used to extract
/// node values in case of errors, allowing for better recovery.
pub trait TryAddNodes: Data {
    /// The associated type for errors that can occur when adding nodes.
    ///
    /// This type should implement the [`Error`] trait.
    type Error: Error;

    /// Attempts to add new [`Node`] to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the node could not be added, along with the value for recovery.
    ///
    /// [`Error`]: Self::Error
    fn try_add_node(&mut self, node: NodeIn<Self>) -> RecoverableNode<Self>;

    /// Attempts to add new node with the given value to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the node could not be added, along with the value for recovery.
    ///
    /// [`Error`]: Self::Error
    fn try_add_node_value(&mut self, value: Self::NodeValue) -> RecoverableNode<Self> {
        self.try_add_node(Node::new(value))
    }

    /// Attempts to add new node with the *default* value to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the node could not be added, along with the value for recovery.
    ///
    /// [`Error`]: Self::Error
    fn try_add_node_default(&mut self) -> RecoverableNode<Self>
    where
        Self::NodeValue: Default,
    {
        self.try_add_node(Node::default())
    }
}

impl<G: TryAddNodes + ?Sized> TryAddNodes for &mut G {
    type Error = G::Error;

    fn try_add_node(&mut self, node: NodeIn<Self>) -> RecoverableNode<Self> {
        (*self).try_add_node(node)
    }
}

pub const ADD_NODE: &str = "failed to add node";
pub const ADD_NODE_VALUE: &str = "failed to add node with value";
pub const ADD_NODE_DEFAULT: &str = "failed to add node with default value";

/// Represents graphs that can have nodes added to them, where all operations are
/// expected to succeed.
///
/// This trait is implemented for any graph that implements [`TryAddNodes`], and all methods
/// in this trait will panic if the corresponding `try` methods fail.
pub trait AddNodes: TryAddNodes {
    /// Adds new [`Node`] to the graph.
    ///
    /// # Panics
    ///
    /// Panics if [`try_add_node`] fails.
    ///
    /// [`try_add_node`]: TryAddNodes::try_add_node
    fn add_node(&mut self, node: NodeIn<Self>) -> Self::NodeId {
        self.try_add_node(node).expect(ADD_NODE)
    }

    /// Adds new node with the given value to the graph.
    ///
    /// # Panics
    ///
    /// Panics if [`try_add_node_value`] fails.
    ///
    /// [`try_add_node_value`]: TryAddNodes::try_add_node_value
    fn add_node_value(&mut self, value: Self::NodeValue) -> Self::NodeId {
        self.try_add_node_value(value).expect(ADD_NODE_VALUE)
    }

    /// Adds new node with the *default* value to the graph.
    ///
    /// # Panics
    ///
    /// Panics if [`try_add_node_default`] fails.
    ///
    /// [`try_add_node_default`]: TryAddNodes::try_add_node_default
    fn add_node_default(&mut self) -> Self::NodeId
    where
        Self::NodeValue: Default,
    {
        self.try_add_node_default().expect(ADD_NODE_DEFAULT)
    }
}

impl<G: TryAddNodes + ?Sized> AddNodes for G {}

/// Represents graphs that can have edges added to them.
///
/// All methods in this trait return [`Recoverable`] results, which can be used to extract
/// edge values in case of errors, allowing for better recovery.
pub trait TryAddEdges: Data {
    /// The associated type for errors that can occur when adding edges.
    ///
    /// This type should implement the [`Error`] trait.
    type Error: Error;

    /// Attempts to add new [`Edge`] to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the edge could not be added, along with the value for recovery.
    ///
    /// [`Error`]: Self::Error
    fn try_add_edge(&mut self, edge: EdgeIn<Self>) -> RecoverableEdge<Self>;

    /// Attempts to add new edge with the given connection and value to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the edge could not be added, along with the value for recovery.
    ///
    /// [`Error`]: Self::Error
    fn try_add_edge_with(
        &mut self,
        connection: Self::Connection,
        value: Self::EdgeValue,
    ) -> RecoverableEdge<Self> {
        self.try_add_edge(Edge::new(connection, value))
    }

    /// Attempts to add new edge with the given connection and the *default* value to the graph.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the edge could not be added, along with the value for recovery.
    ///
    /// [`Error`]: Self::Error
    fn try_add_edge_default(&mut self, connection: Self::Connection) -> RecoverableEdge<Self>
    where
        Self::EdgeValue: Default,
    {
        self.try_add_edge(Edge::new_default(connection))
    }

    /// Attempts to add new edge with the given value that connects the given nodes.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the edge could not be added, along with the value for recovery.
    ///
    /// [`Error`]: Self::Error
    fn try_add_edge_connecting(
        &mut self,
        one: Self::NodeId,
        two: Self::NodeId,
        value: Self::EdgeValue,
    ) -> RecoverableEdge<Self> {
        self.try_add_edge(Edge::connecting(one, two, value))
    }

    /// Attempts to add new edge with the *default* value that connects the given nodes.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the edge could not be added, along with the value for recovery.
    ///
    /// [`Error`]: Self::Error
    fn try_add_edge_connecting_default(
        &mut self,
        one: Self::NodeId,
        two: Self::NodeId,
    ) -> RecoverableEdge<Self>
    where
        Self::EdgeValue: Default,
    {
        self.try_add_edge(Edge::connecting_default(one, two))
    }
}

impl<G: TryAddEdges + ?Sized> TryAddEdges for &mut G {
    type Error = G::Error;

    fn try_add_edge(&mut self, edge: EdgeIn<Self>) -> RecoverableEdge<Self> {
        (*self).try_add_edge(edge)
    }
}

pub const ADD_EDGE: &str = "failed to add edge";
pub const ADD_EDGE_WITH: &str = "failed to add edge with connection and value";
pub const ADD_EDGE_DEFAULT: &str = "failed to add edge with connection and default value";
pub const ADD_EDGE_CONNECTING: &str = "failed to add edge connecting nodes with value";
pub const ADD_EDGE_CONNECTING_DEFAULT: &str =
    "failed to add edge connecting nodes with default value";

/// Represents graphs that can have edges added to them, where all operations are
/// expected to succeed.
///
/// This trait is implemented for any graph that implements [`TryAddEdges`], and all methods
/// in this trait will panic if the corresponding `try` methods fail.
pub trait AddEdges: TryAddEdges {
    /// Adds new [`Edge`] to the graph.
    ///
    /// # Panics
    ///
    /// Panics if [`try_add_edge`] fails.
    ///
    /// [`try_add_edge`]: TryAddEdges::try_add_edge
    fn add_edge(&mut self, edge: EdgeIn<Self>) -> Self::EdgeId {
        self.try_add_edge(edge).expect(ADD_EDGE)
    }

    /// Adds new edge with the given connection and value to the graph.
    ///
    /// # Panics
    ///
    /// Panics if [`try_add_edge_with`] fails.
    ///
    /// [`try_add_edge_with`]: TryAddEdges::try_add_edge_with
    fn add_edge_with(
        &mut self,
        connection: Self::Connection,
        value: Self::EdgeValue,
    ) -> Self::EdgeId {
        self.try_add_edge_with(connection, value)
            .expect(ADD_EDGE_WITH)
    }

    /// Adds new edge with the given connection and the *default* value to the graph.
    ///
    /// # Panics
    ///
    /// Panics if [`try_add_edge_default`] fails.
    ///
    /// [`try_add_edge_default`]: TryAddEdges::try_add_edge_default
    fn add_edge_default(&mut self, connection: Self::Connection) -> Self::EdgeId
    where
        Self::EdgeValue: Default,
    {
        self.try_add_edge_default(connection)
            .expect(ADD_EDGE_DEFAULT)
    }

    /// Adds new edge with the given value that connects the given nodes.
    ///
    /// # Panics
    ///
    /// Panics if [`try_add_edge_connecting`] fails.
    ///
    /// [`try_add_edge_connecting`]: TryAddEdges::try_add_edge_connecting
    fn add_edge_connecting(
        &mut self,
        one: Self::NodeId,
        two: Self::NodeId,
        value: Self::EdgeValue,
    ) -> Self::EdgeId {
        self.try_add_edge_connecting(one, two, value)
            .expect(ADD_EDGE_CONNECTING)
    }

    /// Adds new edge with the *default* value that connects the given nodes.
    ///
    /// # Panics
    ///
    /// Panics if [`try_add_edge_connecting_default`] fails.
    ///
    /// [`try_add_edge_connecting_default`]: TryAddEdges::try_add_edge_connecting_default
    fn add_edge_connecting_default(&mut self, one: Self::NodeId, two: Self::NodeId) -> Self::EdgeId
    where
        Self::EdgeValue: Default,
    {
        self.try_add_edge_connecting_default(one, two)
            .expect(ADD_EDGE_CONNECTING_DEFAULT)
    }
}

impl<G: TryAddEdges + ?Sized> AddEdges for G {}

trait_aliases! {
    /// Represents graphs that can be built incrementally.
    ///
    /// Implemented for any graph that implements both [`TryAddNodes`] and [`TryAddEdges`].
    #[trait_alias(G)]
    pub trait TryBuild = TryAddNodes + TryAddEdges;

    /// Represents graphs that can be built incrementally, where all operations are
    /// expected to succeed.
    ///
    /// Implemented for any graph that implements both [`AddNodes`] and [`AddEdges`].
    #[trait_alias(G)]
    pub trait Build = AddNodes + AddEdges;
}

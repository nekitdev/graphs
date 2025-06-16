//! Base traits for graphs.

use crate::id::{EdgeTypeId, NodeTypeId};

/// Represents the base definition of any graph.
pub trait Base {
    /// The associated type for node identifiers.
    type NodeId: NodeTypeId;

    /// The associated type for edge identifiers.
    type EdgeId: EdgeTypeId;
}

impl<G: Base + ?Sized> Base for &G {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<G: Base + ?Sized> Base for &mut G {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

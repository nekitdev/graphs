//! Base traits for graphs.

use crate::id::Id;

/// Represents the base definition of any graph.
pub trait Base {
    /// The associated type for node identifiers.
    type NodeId: Id;

    /// The associated type for edge identifiers.
    type EdgeId: Id;
}

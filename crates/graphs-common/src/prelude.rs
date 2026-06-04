#[doc(inline)]
pub use crate::{
    connections::Kinded,
    control::{Control, Flow},
    control_flow,
    err_or::ErrOr,
    id::{DefaultEdgeId, DefaultNodeId, EdgeId, NodeId, edge_id, node_id},
    index::{DefaultEdgeIndex, DefaultNodeIndex, EdgeIndex, NodeIndex, edge_index, node_index},
    next::Next,
    record::{Record, Recorder},
    size::ConstSize,
    time::Time,
};

#[doc(inline)]
pub use crate::dfs::prelude::*;

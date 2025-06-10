use crate::{
    base::Base,
    count::{EdgeCount, NodeCount},
};

pub trait NodeIndex: Base {
    fn node_bound(&self) -> usize;
    fn node_index(&self, id: Self::NodeId) -> usize;
    fn node_id(&self, index: usize) -> Self::NodeId;
}

pub trait EdgeIndex: Base {
    fn edge_bound(&self) -> usize;
    fn edge_index(&self, id: Self::EdgeId) -> usize;
    fn edge_id(&self, index: usize) -> Self::EdgeId;
}

pub trait NodeCompact: NodeIndex + NodeCount {}
pub trait EdgeCompact: EdgeIndex + EdgeCount {}

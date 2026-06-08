use trait_aliases::trait_aliases;

use crate::data::Data;

#[derive(Debug, Clone)]
pub struct Removed<N, E: IntoIterator> {
    pub node: N,
    pub edges: E,
}

pub type RemovedIn<G> = Removed<<G as Data>::NodeValue, <G as RemoveNode>::EdgeValues>;

pub trait RemoveNode: Data {
    type EdgeValues: IntoIterator<Item = Self::EdgeValue>;

    fn remove_node(&mut self, node: Self::NodeId) -> Option<RemovedIn<Self>>;
}

pub trait RemoveEdge: Data {
    fn remove_edge(&mut self, edge: Self::EdgeId) -> Option<Self::EdgeValue>;
}

trait_aliases! {
    #[trait_alias(G)]
    pub trait Remove = RemoveNode + RemoveEdge;
}

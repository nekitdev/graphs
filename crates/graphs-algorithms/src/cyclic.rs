use graphs_core::{
    algorithms::Algorithm,
    base::Undirected,
    connections::Connection,
    identifiers::NodeIdentifiers,
    indexed::NodeIndexed,
    neighbors::Neighbors,
    references::{EdgeRef, EdgeReferences},
    visit::Visit,
};
use graphs_union_find::vec::UnionFind;

use crate::cycles::find;

pub fn is_cyclic_undirected<G: NodeIndexed + EdgeReferences + Undirected>(graph: G) -> bool {
    let mut edge_sets = UnionFind::new(graph.node_bound());

    for edge in graph.edge_references() {
        let (one, two) = edge.connection().parts();

        let (one_index, two_index) = (graph.node_index(one), graph.node_index(two));

        if edge_sets.union(one_index, two_index).is_already() {
            return true;
        }
    }

    false
}

pub fn is_cyclic<G: NodeIdentifiers + Neighbors + Visit>(graph: G) -> bool {
    find(graph).is_some()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CyclicUndirected;

impl<G: NodeIndexed + EdgeReferences + Undirected> Algorithm<G> for CyclicUndirected {
    type Output = bool;

    fn perform(&mut self, graph: G) -> Self::Output {
        is_cyclic_undirected(graph)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cyclic;

impl<G: NodeIdentifiers + Neighbors + Visit> Algorithm<G> for Cyclic {
    type Output = bool;

    fn perform(&mut self, graph: G) -> Self::Output {
        is_cyclic(graph)
    }
}

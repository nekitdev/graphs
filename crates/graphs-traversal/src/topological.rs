#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use graphs_core::{
    adapters::Adapt,
    base::Base,
    id::NodeTypeId,
    identifiers::NodeIdentifiers,
    neighbors::{DirectedNeighbors, Neighbors},
    visit::{Visit, Visitor},
    walk::{Walk, Walker},
};

pub struct Topological<N: NodeTypeId, V: Visitor<N>> {
    visit: Vec<N>,
    ordered: V,
}

impl<N: NodeTypeId, V: Visitor<N>> Topological<N, V> {
    pub fn new<G>(graph: G) -> Self
    where
        G: Visit<NodeId = N, Visitor = V>
            + NodeIdentifiers<NodeId = N>
            + DirectedNeighbors<NodeId = N>,
    {
        let mut topological = Self::empty(&graph);

        topological.prepare(graph);

        topological
    }

    pub fn empty<G: Visit<NodeId = N, Visitor = V>>(graph: G) -> Self {
        Self {
            visit: Vec::new(),
            ordered: graph.build_visitor(),
        }
    }

    pub fn prepare<G: NodeIdentifiers<NodeId = N> + DirectedNeighbors<NodeId = N>>(
        &mut self,
        graph: G,
    ) {
        self.visit.extend(
            graph
                .node_identifiers() // select isolated and source nodes
                .filter(|node_ref| !graph.has_incoming(node_ref.clone())),
        );
    }

    pub fn reset<G: Visit<NodeId = N, Visitor = V>>(&mut self, graph: G) {
        self.visit.clear();

        graph.reset_visitor(&mut self.ordered);
    }

    pub fn next<G: Visit<NodeId = N, Visitor = V> + DirectedNeighbors<NodeId = N>>(
        &mut self,
        graph: G,
    ) -> Option<N> {
        while let Some(node) = self.visit.pop() {
            if self.ordered.visit(node) {
                for neighbor in graph.neighbors(node) {
                    if graph
                        .as_reversed()
                        .neighbors(neighbor)
                        .all(|other| self.ordered.was_visited(other))
                    {
                        self.visit.push(neighbor);
                    }
                }

                return Some(node);
            }
        }

        None
    }
}

impl<G: Visit + NodeIdentifiers + DirectedNeighbors> Walker<G>
    for Topological<G::NodeId, G::Visitor>
{
    type Item = G::NodeId;

    fn walk_next(&mut self, context: &G) -> Option<Self::Item> {
        self.next(context)
    }
}

pub type TopologicalOf<G> = Topological<<G as Base>::NodeId, <G as Visit>::Visitor>;

pub type TopologicalIter<'g, G> = Walk<'g, G, TopologicalOf<G>>;

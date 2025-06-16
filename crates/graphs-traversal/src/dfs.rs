#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use graphs_core::{
    base::Base,
    id::NodeTypeId,
    neighbors::Neighbors,
    visit::{Visit, Visitor},
    walk::{Walk, Walker},
};

pub struct Dfs<N: NodeTypeId, V: Visitor<N>> {
    stack: Vec<N>,
    discovered: V,
}

impl<N: NodeTypeId, V: Visitor<N>> Dfs<N, V> {
    pub fn empty<G: Visit<NodeId = N, Visitor = V>>(graph: G) -> Self {
        Self {
            stack: Vec::new(),
            discovered: graph.build_visitor(),
        }
    }

    pub fn new<G: Visit<NodeId = N, Visitor = V>>(graph: G, start: N) -> Self {
        let mut dfs = Dfs::empty(graph);

        dfs.move_to(start);

        dfs
    }

    pub fn reset<G: Visit<NodeId = N, Visitor = V>>(&mut self, graph: G) {
        graph.reset_visitor(&mut self.discovered);

        self.stack.clear();
    }

    pub fn move_to(&mut self, node: N) {
        self.stack.clear();

        self.stack.push(node);
    }

    pub fn next<G: Neighbors<NodeId = N>>(&mut self, graph: G) -> Option<N> {
        while let Some(node) = self.stack.pop() {
            if self.discovered.visit(node) {
                for neighbor in graph.neighbors(node) {
                    if !self.discovered.was_visited(neighbor) {
                        self.stack.push(neighbor);
                    }
                }

                return Some(node);
            }
        }

        None
    }
}

impl<G: Visit + Neighbors> Walker<G> for Dfs<G::NodeId, G::Visitor> {
    type Item = G::NodeId;

    fn walk_next(&mut self, context: &G) -> Option<Self::Item> {
        self.next(context)
    }
}

pub type DfsOf<G> = Dfs<<G as Base>::NodeId, <G as Visit>::Visitor>;

pub type DfsIter<'g, G> = Walk<'g, G, DfsOf<G>>;

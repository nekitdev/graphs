#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use graphs_core::{
    base::Base,
    id::NodeTypeId,
    neighbors::Neighbors,
    visit::{Visit, Visitor},
    walk::{Walk, Walker},
};

pub struct DfsPostOrder<N: NodeTypeId, V: Visitor<N>> {
    stack: Vec<N>,
    discovered: V,
    finished: V,
}

impl<N: NodeTypeId, V: Visitor<N>> DfsPostOrder<N, V> {
    pub fn empty<G: Visit<NodeId = N, Visitor = V>>(graph: G) -> Self {
        Self {
            stack: Vec::new(),
            discovered: graph.build_visitor(),
            finished: graph.build_visitor(),
        }
    }

    pub fn new<G: Visit<NodeId = N, Visitor = V>>(graph: G, start: N) -> Self {
        let mut dfs_post_order = Self::empty(graph);

        dfs_post_order.move_to(start);

        dfs_post_order
    }

    pub fn reset<G: Visit<NodeId = N, Visitor = V>>(&mut self, graph: G) {
        graph.reset_visitor(&mut self.discovered);
        graph.reset_visitor(&mut self.finished);

        self.stack.clear();
    }

    pub fn move_to(&mut self, start: N) {
        self.stack.clear();

        self.stack.push(start);
    }

    pub fn next<G: Neighbors<NodeId = N>>(&mut self, graph: G) -> Option<N> {
        while let Some(node) = self.stack.last().copied() {
            if self.discovered.visit(node) {
                for neighbor in graph.neighbors(node) {
                    if !self.discovered.was_visited(neighbor) {
                        self.stack.push(neighbor);
                    }
                }
            } else {
                self.stack.pop();

                if self.finished.visit(node) {
                    return Some(node);
                }
            }
        }

        None
    }
}

impl<G: Visit + Neighbors> Walker<G> for DfsPostOrder<G::NodeId, G::Visitor> {
    type Item = G::NodeId;

    fn walk_next(&mut self, context: &G) -> Option<Self::Item> {
        self.next(context)
    }
}

pub type DfsPostOrderOf<G> = DfsPostOrder<<G as Base>::NodeId, <G as Visit>::Visitor>;

pub type DfsPostOrderWalk<'g, G> = Walk<'g, G, DfsPostOrderOf<G>>;

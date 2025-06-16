use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::collections::VecDeque;
    } else {
        use alloc::collections::VecDeque;
    }
}

use graphs_core::{
    base::Base,
    id::NodeTypeId,
    neighbors::Neighbors,
    visit::{Visit, Visitor},
    walk::{Walk, Walker},
};

pub struct Bfs<N: NodeTypeId, V: Visitor<N>> {
    queue: VecDeque<N>,
    discovered: V,
}

impl<N: NodeTypeId, V: Visitor<N>> Bfs<N, V> {
    pub fn empty<G: Visit<NodeId = N, Visitor = V>>(graph: G) -> Self {
        Self {
            queue: VecDeque::new(),
            discovered: graph.build_visitor(),
        }
    }

    pub fn new<G: Visit<NodeId = N, Visitor = V>>(graph: G, start: N) -> Self {
        let mut bfs = Bfs::empty(graph);

        bfs.move_to(start);

        bfs
    }

    pub fn reset<G: Visit<NodeId = N, Visitor = V>>(&mut self, graph: G) {
        graph.reset_visitor(&mut self.discovered);

        self.queue.clear();
    }

    pub fn move_to(&mut self, node: N) {
        self.queue.clear();

        if self.discovered.visit(node) {
            self.queue.push_back(node);
        }
    }

    pub fn next<G: Neighbors<NodeId = N>>(&mut self, graph: G) -> Option<N> {
        let node = self.queue.pop_front()?;

        for neighbor in graph.neighbors(node) {
            if self.discovered.visit(neighbor) {
                self.queue.push_back(neighbor);
            }
        }

        Some(node)
    }
}

impl<G: Visit + Neighbors> Walker<G> for Bfs<G::NodeId, G::Visitor> {
    type Item = G::NodeId;

    fn walk_next(&mut self, context: &G) -> Option<Self::Item> {
        self.next(context)
    }
}

pub type BfsOf<G> = Bfs<<G as Base>::NodeId, <G as Visit>::Visitor>;

pub type BfsIter<'g, G> = Walk<'g, G, BfsOf<G>>;

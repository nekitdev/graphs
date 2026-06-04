//! Depth-first search (DFS).

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use graphs_core::{
    base::Base,
    id::NodeTypeId,
    neighbors::Neighbors,
    visit::{Visit, Visitor},
    walk::{Walk, Walker},
};

/// Represents iterative depth-first search (DFS).
pub struct Dfs<N: NodeTypeId, V: Visitor<N>> {
    stack: Vec<N>,
    discovered: V,
}

impl<N: NodeTypeId, V: Visitor<N>> Dfs<N, V> {
    /// Constructs [`Self`] from parts, namely the DFS *stack* and *discovered* visitor.
    pub const fn from_parts(stack: Vec<N>, discovered: V) -> Self {
        Self { stack, discovered }
    }

    pub fn empty<G: Visit<NodeId = N, Visitor = V>>(graph: G) -> Self {
        Self::from_parts(Vec::new(), graph.build_visitor())
    }

    pub fn new<G: Visit<NodeId = N, Visitor = V>>(graph: G, start: N) -> Self {
        Self::from_parts(vec![start], graph.build_visitor())
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
            if self.discovered.visit(node).is_newly() {
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

pub type DfsOn<G> = Dfs<<G as Base>::NodeId, <G as Visit>::Visitor>;

pub type DfsWalk<'g, G> = Walk<'g, G, DfsOn<G>>;

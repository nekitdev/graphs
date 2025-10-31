use graphs_core::{
    identifiers::NodeIdentifiers,
    neighbors::{DirectedNeighbors, Neighbors},
    visit::Visit,
    walk::Walker,
};

use crate::{
    bfs::{Bfs, BfsOf, BfsWalk},
    dfs::{Dfs, DfsOf, DfsWalk},
    dfs_post_order::{DfsPostOrder, DfsPostOrderOf, DfsPostOrderWalk},
    topological::{Topological, TopologicalOf, TopologicalWalk},
};

pub trait Traverse: Visit {
    fn dfs(&self, start: Self::NodeId) -> DfsOf<Self> {
        Dfs::new(self, start)
    }

    fn dfs_post_order(&self, start: Self::NodeId) -> DfsPostOrderOf<Self> {
        DfsPostOrder::new(self, start)
    }

    fn bfs(&self, start: Self::NodeId) -> BfsOf<Self> {
        Bfs::new(self, start)
    }
}

impl<G: Visit + ?Sized> Traverse for G {}

pub trait TraverseWalk: Traverse + Neighbors + Sized {
    fn dfs_walk(&self, start: Self::NodeId) -> DfsWalk<'_, Self> {
        self.dfs(start).into_walk(self)
    }

    fn dfs_post_order_walk(&self, start: Self::NodeId) -> DfsPostOrderWalk<'_, Self> {
        self.dfs_post_order(start).into_walk(self)
    }

    fn bfs_walk(&self, start: Self::NodeId) -> BfsWalk<'_, Self> {
        self.bfs(start).into_walk(self)
    }
}

impl<G: Traverse + Neighbors> TraverseWalk for G {}

pub trait TraverseTopological: Visit + NodeIdentifiers + DirectedNeighbors {
    fn topological(&self) -> TopologicalOf<Self> {
        Topological::new(self)
    }
}

impl<G: Visit + NodeIdentifiers + DirectedNeighbors + ?Sized> TraverseTopological for G {}

pub trait TraverseTopologicalWalk: TraverseTopological + Sized {
    fn topological_walk(&self) -> TopologicalWalk<'_, Self> {
        self.topological().into_walk(self)
    }
}

impl<G: TraverseTopological> TraverseTopologicalWalk for G {}

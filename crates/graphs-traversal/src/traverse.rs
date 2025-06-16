use graphs_core::{
    identifiers::NodeIdentifiers,
    neighbors::{DirectedNeighbors, Neighbors},
    visit::Visit,
    walk::Walker,
};

use crate::{
    bfs::{Bfs, BfsIter, BfsOf},
    dfs::{Dfs, DfsIter, DfsOf},
    dfs_post_order::{DfsPostOrder, DfsPostOrderIter, DfsPostOrderOf},
    topological::{Topological, TopologicalIter, TopologicalOf},
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

pub trait TraverseIter: Traverse + Neighbors + Sized {
    fn dfs_iter(&self, start: Self::NodeId) -> DfsIter<'_, Self> {
        self.dfs(start).into_walk(self)
    }

    fn dfs_post_order_iter(&self, start: Self::NodeId) -> DfsPostOrderIter<'_, Self> {
        self.dfs_post_order(start).into_walk(self)
    }

    fn bfs_iter(&self, start: Self::NodeId) -> BfsIter<'_, Self> {
        self.bfs(start).into_walk(self)
    }
}

impl<G: Traverse + Neighbors> TraverseIter for G {}

pub trait TraverseTopological: Visit + NodeIdentifiers + DirectedNeighbors {
    fn topological(&self) -> TopologicalOf<Self> {
        Topological::new(self)
    }
}

impl<G: Visit + NodeIdentifiers + DirectedNeighbors + ?Sized> TraverseTopological for G {}

pub trait TraverseTopologicalIter: TraverseTopological + Sized {
    fn topological_iter(&self) -> TopologicalIter<'_, Self> {
        self.topological().into_walk(self)
    }
}

impl<G: TraverseTopological> TraverseTopologicalIter for G {}

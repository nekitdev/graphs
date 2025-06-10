pub trait NodeCount {
    /// Returns the number of nodes in this graph.
    fn node_count(&self) -> usize;
}

pub trait EdgeCount {
    /// Returns the number of edges in this graph.
    fn edge_count(&self) -> usize;
}

pub struct Counts {
    pub nodes: usize,
    pub edges: usize,
}

impl Counts {
    pub const fn new(nodes: usize, edges: usize) -> Self {
        Self { nodes, edges }
    }
}

pub trait Count: NodeCount + EdgeCount {
    fn count(&self) -> Counts {
        Counts::new(self.node_count(), self.edge_count())
    }
}

impl<T: NodeCount + EdgeCount> Count for T {}

pub trait NodeCapacity {
    fn node_capacity(&self) -> usize;
}

pub trait EdgeCapacity {
    fn edge_capacity(&self) -> usize;
}

pub struct Capacities {
    pub nodes: usize,
    pub edges: usize,
}

impl Capacities {
    pub const fn new(nodes: usize, edges: usize) -> Self {
        Self { nodes, edges }
    }
}

pub trait Capacity: NodeCapacity + EdgeCapacity {
    fn capacity(&self) -> Capacities {
        Capacities::new(self.node_capacity(), self.edge_capacity())
    }
}

impl<T: NodeCapacity + EdgeCapacity> Capacity for T {}

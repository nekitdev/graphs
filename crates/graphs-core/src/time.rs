use crate::id::{DefaultNodeId, NodeTypeId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Time {
    value: usize,
}

pub const START: usize = 0;

impl Time {
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    pub const fn start() -> Self {
        Self::START
    }

    pub const fn copy(&self) -> Self {
        Self::new(self.value)
    }

    pub const fn increment(&mut self) -> Self {
        let copy = self.copy();

        self.value += 1;

        copy
    }

    pub const fn get(self) -> usize {
        self.value
    }

    pub const START: Self = Self::new(START);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Timed<N: NodeTypeId = DefaultNodeId> {
    pub node: N,
    pub time: Time,
}

impl<N: NodeTypeId> Timed<N> {
    pub const fn new(node: N, time: Time) -> Self {
        Self { node, time }
    }

    pub const fn copy(&self) -> Self {
        Self::new(self.node, self.time.copy())
    }
}

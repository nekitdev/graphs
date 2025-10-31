use crate::id::{DefaultNodeId, NodeTypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    value: usize,
}

pub const START: usize = 0;

impl Default for Time {
    fn default() -> Self {
        Self::start()
    }
}

impl Time {
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    pub const fn start() -> Self {
        Self::START
    }

    pub const fn increment(&mut self) -> Self {
        let copy = *self;

        self.increment_in_place();

        copy
    }

    pub const fn increment_in_place(&mut self) {
        self.value = self.value.saturating_add(1);
    }

    pub const fn get(self) -> usize {
        self.value
    }

    pub const START: Self = Self::new(START);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timed<N: NodeTypeId = DefaultNodeId> {
    pub node: N,
    pub time: Time,
}

impl<N: NodeTypeId> Default for Timed<N> {
    fn default() -> Self {
        Self::limit()
    }
}

impl<N: NodeTypeId> Timed<N> {
    pub const fn new(node: N, time: Time) -> Self {
        Self { node, time }
    }

    pub const fn limit_at(time: Time) -> Self {
        Self::new(N::LIMIT, time)
    }

    pub const fn limit() -> Self {
        Self::limit_at(Time::start())
    }
}

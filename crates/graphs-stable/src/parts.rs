use std::mem::replace;

use graphs_core::{
    connections::Kinded,
    count::Counts,
    index::{DefaultUntypedIndex, EdgeIndex, NodeIndex, UntypedIndex},
    kinds::DefaultKind,
    limit::Limited,
};

pub struct Free<I: UntypedIndex = DefaultUntypedIndex> {
    pub node: NodeIndex<I>,
    pub edge: EdgeIndex<I>,
}

impl<I: UntypedIndex> Limited for Free<I> {
    const LIMIT: Self = Self::new(NodeIndex::LIMIT, EdgeIndex::LIMIT);
}

impl<I: UntypedIndex> Free<I> {
    pub const fn new(node: NodeIndex<I>, edge: EdgeIndex<I>) -> Self {
        Self { node, edge }
    }

    pub const fn reset(&mut self) -> Self {
        replace(self, Self::LIMIT)
    }
}

pub struct Info<I: UntypedIndex = DefaultUntypedIndex> {
    pub count: Counts,
    pub free: Free<I>,
}

impl<I: UntypedIndex> Info<I> {
    pub const INITIAL: Self = Self::new(Counts::NULL, Free::LIMIT);

    pub const fn new(count: Counts, free: Free<I>) -> Self {
        Self { count, free }
    }
}

pub type Connection<I = DefaultUntypedIndex, K = DefaultKind> = Kinded<NodeIndex<I>, K>;

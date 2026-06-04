use std::mem::replace;

use graphs_common::{
    connections::Kinded,
    index::{EdgeIndex, NodeIndex},
};

use graphs_core::{
    cardinality::Cardinality,
    index::{DefaultUntypedIndex, UntypedIndex},
    kinds::DefaultKind,
    sentinel::Sentinel,
};

pub struct Free<I: UntypedIndex = DefaultUntypedIndex> {
    pub node: NodeIndex<I>,
    pub edge: EdgeIndex<I>,
}

impl<I: UntypedIndex> Clone for Free<I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I: UntypedIndex> Copy for Free<I> {}

impl<I: UntypedIndex> Sentinel for Free<I> {
    const SENTINEL: Self = Self::new(NodeIndex::SENTINEL, EdgeIndex::SENTINEL);

    fn is_sentinel(&self) -> bool {
        self.node.is_sentinel() && self.edge.is_sentinel()
    }
}

impl<I: UntypedIndex> Free<I> {
    pub const fn new(node: NodeIndex<I>, edge: EdgeIndex<I>) -> Self {
        Self { node, edge }
    }

    pub const fn reset(&mut self) -> Self {
        replace(self, Self::SENTINEL)
    }
}

pub struct Info<I: UntypedIndex = DefaultUntypedIndex> {
    pub cardinality: Cardinality,
    pub free: Free<I>,
}

impl<I: UntypedIndex> Clone for Info<I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I: UntypedIndex> Copy for Info<I> {}

impl<I: UntypedIndex> Info<I> {
    pub const INITIAL: Self = Self::new(Cardinality::ZERO, Free::SENTINEL);

    pub const fn new(cardinality: Cardinality, free: Free<I>) -> Self {
        Self { cardinality, free }
    }
}

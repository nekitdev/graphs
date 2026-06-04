use core::{marker::PhantomData, ops::Range};

use graphs_common::index::{EdgeIndex, NodeIndex};
use graphs_core::index::{DefaultUntypedIndex, Index, UntypedIndex};

pub struct NodeIndices<I: UntypedIndex = DefaultUntypedIndex> {
    range: Range<usize>,
    index: PhantomData<I>,
}

impl<I: UntypedIndex> NodeIndices<I> {
    pub const fn new(order: usize) -> Self {
        Self {
            range: 0..order,
            index: PhantomData,
        }
    }
}

impl<I: UntypedIndex> Iterator for NodeIndices<I> {
    type Item = NodeIndex<I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(NodeIndex::of)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<I: UntypedIndex> DoubleEndedIterator for NodeIndices<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(NodeIndex::of)
    }
}

impl<I: UntypedIndex> ExactSizeIterator for NodeIndices<I> {}

pub struct EdgeIndices<I: UntypedIndex = DefaultUntypedIndex> {
    range: Range<usize>,
    index: PhantomData<I>,
}

impl<I: UntypedIndex> EdgeIndices<I> {
    pub const fn new(size: usize) -> Self {
        Self {
            range: 0..size,
            index: PhantomData,
        }
    }
}

impl<I: UntypedIndex> Iterator for EdgeIndices<I> {
    type Item = EdgeIndex<I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(EdgeIndex::of)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<I: UntypedIndex> DoubleEndedIterator for EdgeIndices<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(EdgeIndex::of)
    }
}

impl<I: UntypedIndex> ExactSizeIterator for EdgeIndices<I> {}

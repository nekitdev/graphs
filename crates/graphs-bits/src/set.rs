#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

use graphs_core::{keys::NodeTypeIndex, visit::Visitor};

use crate::{
    block::{BitBlock, DefaultBlock},
    vec::BitVec,
};

pub struct BitSet<B: BitBlock = DefaultBlock> {
    pub(crate) bits: BitVec<B>,
}

impl<B: BitBlock> Default for BitSet<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: BitBlock> BitSet<B> {
    pub const fn new() -> Self {
        Self::construct(BitVec::new())
    }

    pub const fn construct(bits: BitVec<B>) -> Self {
        Self { bits }
    }

    pub fn insert(&mut self, value: usize) -> bool {
        todo!()
    }

    pub fn contains(&self, value: usize) -> bool {
        todo!()
    }

    pub fn remove(&mut self, value: usize) -> bool {
        todo!()
    }
}

impl<B: BitBlock, N: NodeTypeIndex> Visitor<N> for BitSet<B> {
    fn visit(&mut self, node: N) -> bool {
        self.insert(node.index())
    }

    fn was_visited(&self, node: N) -> bool {
        self.contains(node.index())
    }

    fn unvisit(&mut self, node: N) -> bool {
        self.remove(node.index())
    }
}

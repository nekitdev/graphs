#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

use core::slice::{Iter, IterMut};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::{
    block::{BitBlock, DefaultBlock, Overflow, bits, try_bits},
    capacity::{Bits, Blocks, Capacity},
};

pub const CAPACITY_OVERFLOW: &str = "capacity overflow";

#[derive(Debug, Error)]
#[error("{CAPACITY_OVERFLOW}")]
pub struct CapacityOverflow;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitVec<B: BitBlock = DefaultBlock> {
    pub(crate) storage: Vec<B>,
    pub(crate) bits: usize,
}

impl<B: BitBlock> Clone for BitVec<B> {
    fn clone(&self) -> Self {
        unsafe { Self::construct_unchecked(self.storage.clone(), self.bits) }
    }
}

impl<B: BitBlock> Default for BitVec<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: BitBlock> FromIterator<bool> for BitVec<B> {
    fn from_iter<I: IntoIterator<Item = bool>>(iterable: I) -> Self {
        let mut bits = Self::new();

        bits.extend(iterable);

        bits
    }
}

impl<B: BitBlock> Extend<bool> for BitVec<B> {
    fn extend<I: IntoIterator<Item = bool>>(&mut self, iterable: I) {
        let iterator = iterable.into_iter();

        let (lower, _) = iterator.size_hint();

        self.reserve_bits(lower);

        // iterator.for_each(|bit| self.push(bit));
    }
}

impl BitVec {
    pub const fn default_new() -> Self {
        Self::new()
    }

    pub fn default_repeat(bit: bool, count: usize) -> Self {
        Self::repeat(bit, count)
    }
}

impl<B: BitBlock> BitVec<B> {
    pub const fn new() -> Self {
        Self::construct(Vec::new())
    }

    pub const fn len(&self) -> usize {
        self.bits
    }

    pub const fn try_capacity(&self) -> Result<usize, Overflow> {
        try_bits::<B>(self.storage.capacity())
    }

    pub const fn capacity(&self) -> Option<usize> {
        bits::<B>(self.storage.capacity())
    }

    pub fn repeat(bit: bool, count: usize) -> Self {
        let blocks = B::blocks(count);

        let block = if bit { B::ALL } else { B::ZERO };

        let storage = vec![block; blocks];

        let mut bits = unsafe { Self::construct_unchecked(storage, count) };

        bits.fix_last_block();

        bits
    }

    pub fn from_fn<F: FnMut(usize) -> bool>(function: F, length: usize) -> Self {
        (0..length).map(function).collect()
    }

    pub fn with_capacity(capacity: Capacity) -> Self {
        match capacity {
            Bits(bits) => Self::with_bits_capacity(bits),
            Blocks(blocks) => Self::with_blocks_capacity(blocks),
        }
    }

    pub fn with_bits_capacity(bits: usize) -> Self {
        Self::with_blocks_capacity(B::blocks(bits))
    }

    pub fn with_blocks_capacity(blocks: usize) -> Self {
        Self::construct(Vec::with_capacity(blocks))
    }

    pub const fn construct(storage: Vec<B>) -> Self {
        // SAFETY: constructing with `bits` set to `0` is always safe
        unsafe { Self::construct_unchecked(storage, 0) }
    }

    pub const unsafe fn construct_unchecked(storage: Vec<B>, bits: usize) -> Self {
        Self { storage, bits }
    }
}

impl<B: BitBlock> BitVec<B> {
    pub fn blocks(&self) -> Iter<'_, B> {
        self.storage.iter()
    }

    pub fn blocks_mut(&mut self) -> IterMut<'_, B> {
        self.storage.iter_mut()
    }

    pub fn into_blocks(self) -> Vec<B> {
        self.storage
    }

    pub fn reserve(&mut self, additional: Capacity) {
        self.try_reserve(additional).expect(CAPACITY_OVERFLOW);
    }

    pub fn reserve_bits(&mut self, bits: usize) {
        self.try_reserve_bits(bits).expect(CAPACITY_OVERFLOW);
    }

    pub fn reserve_blocks(&mut self, blocks: usize) {
        self.try_reserve_blocks(blocks).expect(CAPACITY_OVERFLOW);
    }

    pub fn try_reserve(&mut self, additional: Capacity) -> Result<(), CapacityOverflow> {
        match additional {
            Bits(bits) => self.try_reserve_bits(bits),
            Blocks(blocks) => self.try_reserve_blocks(blocks),
        }
    }

    pub fn try_reserve_bits(&mut self, bits: usize) -> Result<(), CapacityOverflow> {
        self.try_reserve_blocks(B::blocks(bits))
    }

    pub fn try_reserve_blocks(&mut self, blocks: usize) -> Result<(), CapacityOverflow> {
        todo!()
    }

    pub fn shrink_to_fit(&mut self) {
        self.storage.shrink_to_fit();
    }

    // pub fn get<I: BitIndex<Self>>(&self, index: I) -> Option<&I::Output> {
    //     index.get(self)
    // }

    // pub fn get_mut<I: BitIndex<Self>>(&mut self, index: I) -> Option<&mut I::Output> {
    //     index.get_mut(self)
    // }

    // pub unsafe fn get_unchecked<I: BitIndex<Self>>(&self, index: I) -> &I::Output {
    //     unsafe { index.get_unchecked(self) }
    // }

    // pub unsafe fn get_unchecked_mut<I: BitIndex<Self>>(&self, index: I) -> &mut I::Output {
    //     todo!()
    // }
}

impl<B: BitBlock> BitVec<B> {
    pub fn all(&self) -> bool {
        let mut iterator = self.blocks().copied();

        let Some(last) = iterator.next_back() else {
            return true;
        };

        last == self.extra_mask() && iterator.all(B::is_all)
    }

    pub fn any(&self) -> bool {
        !self.none()
    }

    pub fn none(&self) -> bool {
        self.blocks().copied().all(B::is_zero)
    }
}

pub(crate) struct LastBlockRef<'b, B: BitBlock = DefaultBlock> {
    block: &'b B,
    extra: usize,
}

impl<'b, B: BitBlock> LastBlockRef<'b, B> {
    pub(crate) const fn new(block: &'b B, extra: usize) -> Self {
        Self { block, extra }
    }

    pub(crate) fn is_fine(&self) -> bool {
        *self.block & B::inverse_mask(self.extra) == B::ZERO
    }

    pub fn fixed(&self) -> B {
        *self.block & B::mask(self.extra)
    }

    pub fn fixed_inverse(&self) -> B {
        *self.block | B::inverse_mask(self.extra)
    }
}

pub(crate) struct LastBlockMut<'b, B: BitBlock = DefaultBlock> {
    block: &'b mut B,
    extra: usize,
}

impl<'b, B: BitBlock> LastBlockMut<'b, B> {
    pub(crate) const fn new(block: &'b mut B, extra: usize) -> Self {
        Self { block, extra }
    }

    pub(crate) const fn as_ref(&self) -> LastBlockRef<'_, B> {
        LastBlockRef::new(self.block, self.extra)
    }

    pub fn is_fine(&self) -> bool {
        self.as_ref().is_fine()
    }

    pub fn fix(&mut self) {
        *self.block = self.as_ref().fixed();
    }

    pub fn fix_inverse(&mut self) {
        *self.block = self.as_ref().fixed_inverse();
    }
}

impl<B: BitBlock> BitVec<B> {
    pub(crate) const fn extra(&self) -> usize {
        self.len() % B::BITS
    }

    pub fn extra_mask(&self) -> B {
        B::mask(self.extra())
    }

    pub(crate) fn is_last_block_fine(&self) -> bool {
        self.last_block().is_none_or(|block| block.is_fine())
    }

    pub(crate) fn fix_last_block(&mut self) {
        if let Some(mut last_block) = self.last_block_mut() {
            last_block.fix();
        }
    }

    pub(crate) fn fix_last_block_inverse(&mut self) {
        if let Some(mut last_block) = self.last_block_mut() {
            last_block.fix_inverse();
        }
    }

    pub(crate) fn last_block(&self) -> Option<LastBlockRef<'_, B>> {
        let extra = self.extra();

        if extra != 0 {
            self.storage
                .last()
                .map(|block| LastBlockRef::new(block, extra))
        } else {
            None
        }
    }

    pub(crate) fn last_block_mut(&mut self) -> Option<LastBlockMut<'_, B>> {
        let extra = self.extra();

        if extra != 0 {
            self.storage
                .last_mut()
                .map(|block| LastBlockMut::new(block, extra))
        } else {
            None
        }
    }
}

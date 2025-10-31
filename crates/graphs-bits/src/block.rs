use core::{
    hash::Hash,
    ops::{BitAnd, BitOr, BitXor, Not},
};

use thiserror::Error;

pub const OVERFLOW: &str = "overflow";

#[derive(Debug, Error)]
#[error("{OVERFLOW}")]
pub struct Overflow;

pub type Byte = u8;
pub type Bits = u8;
pub type Base = u32;

pub const BYTE_BITS: Bits = Byte::BITS as Bits;

pub const fn blocks<B: BitBlock>(bits: usize) -> usize {
    bits.div_ceil(B::BITS as usize)
}

pub const fn bits<B: BitBlock>(blocks: usize) -> Option<usize> {
    blocks.checked_mul(B::BITS as usize)
}

pub const fn try_bits<B: BitBlock>(blocks: usize) -> Result<usize, Overflow> {
    if let Some(bits) = bits::<B>(blocks) {
        Ok(bits)
    } else {
        Err(Overflow)
    }
}

pub const fn div_rem<B: BitBlock>(bit: usize) -> (usize, usize) {
    (bit / B::BITS as usize, bit % B::BITS as usize)
}

pub trait BitBlock:
    Copy
    + Eq
    + Ord
    + Hash
    + Not<Output = Self>
    + BitAnd<Self, Output = Self>
    + BitOr<Self, Output = Self>
    + BitXor<Self, Output = Self>
{
    const BITS: Bits;

    const ZERO: Self;
    const ONE: Self;
    const ALL: Self;

    fn from_byte(byte: Byte) -> Self;

    fn checked_add(self, other: Self) -> Option<Self>;
    fn checked_sub(self, other: Self) -> Option<Self>;

    fn checked_shift_left(self, shift: Bits) -> Option<Self>;
    fn checked_shift_right(self, shift: Bits) -> Option<Self>;

    fn blocks(bits: usize) -> usize {
        blocks::<Self>(bits)
    }

    fn bits(blocks: usize) -> Option<usize> {
        bits::<Self>(blocks)
    }

    fn div_rem(bit: usize) -> (usize, usize) {
        div_rem::<Self>(bit)
    }

    fn flag(shift: Bits) -> Option<Self> {
        Self::ONE.checked_shift_left(shift)
    }

    fn mask(bits: Bits) -> Option<Self> {
        let flag = Self::flag(bits)?;

        flag.checked_sub(Self::ONE)
    }

    fn inverse_mask(bits: Bits) -> Option<Self> {
        let mask = Self::mask(bits)?;

        Some(!mask)
    }

    fn get(self, bit: Bits) -> Option<bool> {
        let flag = Self::flag(bit)?;

        Some((self & flag).is_non_zero())
    }

    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    fn is_non_zero(self) -> bool {
        !self.is_zero()
    }

    fn is_all(self) -> bool {
        self == Self::ALL
    }
}

pub type DefaultBlock = Base;

macro_rules! impl_primitive {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::block::BitBlock for $type {
                const BITS: Bits = <$type>::BITS as Bits;

                const ZERO: Self = 0;
                const ONE: Self = 1;
                const ALL: Self = !Self::ZERO;

                fn from_byte(byte: Byte) -> Self {
                    byte as $type
                }

                fn checked_add(self, other: Self) -> Option<Self> {
                    self.checked_add(other)
                }

                fn checked_sub(self, other: Self) -> Option<Self> {
                    self.checked_sub(other)
                }

                fn checked_shift_left(self, shift: Bits) -> Option<Self> {
                    self.checked_shl(shift as $crate::block::Base)
                }

                fn checked_shift_right(self, shift: Bits) -> Option<Self> {
                    self.checked_shr(shift as $crate::block::Base)
                }
            }
        )+
    };
}

impl_primitive!(u8, u16, u32, u64, u128, usize);

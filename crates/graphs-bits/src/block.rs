use core::{
    hash::Hash,
    ops::{BitAnd, BitOr, BitXor, Not},
};

pub type Byte = u8;
pub type Base = u32;
pub type Bits = usize;

pub const OVERFLOW: &str = "bits overflow";

pub const BYTE_BITS: Bits = Byte::BITS as Bits;

pub const fn blocks<B: BitBlock>(bits: usize) -> usize {
    bits.div_ceil(B::BITS as usize)
}

pub const fn try_bits<B: BitBlock>(blocks: usize) -> Option<usize> {
    blocks.checked_mul(B::BITS as usize)
}

pub const fn bits<B: BitBlock>(blocks: usize) -> usize {
    try_bits::<B>(blocks).expect(OVERFLOW)
}

pub const fn div_rem<B: BitBlock>(bit: usize) -> (usize, usize) {
    (bit / B::BITS as usize, bit % B::BITS as usize)
}

pub trait BitBlock:
    Copy
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

    fn checked_add(self, other: Self) -> Option<Self>;
    fn checked_sub(self, other: Self) -> Option<Self>;

    fn checked_shift_left(self, shift: Bits) -> Option<Self>;
    fn checked_shift_right(self, shift: Bits) -> Option<Self>;

    fn blocks(bits: usize) -> usize {
        blocks::<Self>(bits)
    }

    fn try_bits(blocks: usize) -> Option<usize> {
        try_bits::<Self>(blocks)
    }

    fn bits(blocks: usize) -> usize {
        bits::<Self>(blocks)
    }

    fn div_rem(bit: usize) -> (usize, usize) {
        div_rem::<Self>(bit)
    }

    fn try_flag(shift: Bits) -> Option<Self> {
        Self::ONE.checked_shift_left(shift)
    }

    fn flag(shift: Bits) -> Self {
        Self::try_flag(shift).expect(OVERFLOW)
    }

    fn try_mask(bits: Bits) -> Option<Self> {
        let flag = Self::try_flag(bits)?;

        flag.checked_sub(Self::ONE)
    }

    fn mask(bits: Bits) -> Self {
        Self::try_mask(bits).expect(OVERFLOW)
    }

    fn try_inverse_mask(bits: Bits) -> Option<Self> {
        let mask = Self::try_mask(bits)?;

        Some(!mask)
    }

    fn inverse_mask(bits: Bits) -> Self {
        Self::try_inverse_mask(bits).expect(OVERFLOW)
    }

    fn try_get(self, bit: Bits) -> Option<bool> {
        let flag = Self::try_flag(bit)?;

        Some((self & flag).is_non_zero())
    }

    fn get(self, bit: Bits) -> bool {
        self.try_get(bit).expect(OVERFLOW)
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

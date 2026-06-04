//! Size-related utilities.

pub trait ConstSize: Sized {
    const SIZE: usize = size_of::<Self>();
    const ZERO: bool = Self::SIZE == 0;
}

impl<T> ConstSize for T {}

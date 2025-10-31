//! Size-related utilities.

pub trait Size: Sized {
    const SIZE: usize = size_of::<Self>();
    const ZERO: bool = Self::SIZE == 0;
}

impl<T> Size for T {}

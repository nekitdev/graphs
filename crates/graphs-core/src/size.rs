//! Size-related utilities.

/// Checks if the type `T` has non-zero size.
pub const fn has<T>() -> bool {
    size_of::<T>() != 0
}

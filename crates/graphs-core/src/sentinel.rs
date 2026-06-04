/// Represents types that have *sentinel* values, which are used to represent *invalid*
/// or *uninitialized* values of the type.
pub trait Sentinel: Sized {
    /// The sentinel value of the type.
    const SENTINEL: Self;

    /// Checks if the value is sentinel.
    fn is_sentinel(&self) -> bool;

    /// Checks if the value is regular, that is, not sentinel.
    fn is_regular(&self) -> bool {
        !self.is_sentinel()
    }
}

macro_rules! impl_int_sentinel {
    ($($int: ty),+ $(,)?) => {
        $(
            impl $crate::sentinel::Sentinel for $int {
                const SENTINEL: Self = <$int>::MAX;

                fn is_sentinel(&self) -> bool {
                    *self == Self::SENTINEL
                }
            }
        )+
    };
}

impl_int_sentinel!(u8, u16, u32, u64, u128, usize);

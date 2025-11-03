pub trait Limited: Sized {
    const LIMIT: Self;

    fn is_limit(&self) -> bool
    where
        Self: PartialEq,
    {
        self == &Self::LIMIT
    }
}

macro_rules! impl_limited {
    ($($int: ty),+ $(,)?) => {
        $(
            impl $crate::limit::Limited for $int {
                const LIMIT: Self = <$int>::MAX;
            }
        )+
    };
}

impl_limited!(u8, u16, u32, u64, u128, usize);

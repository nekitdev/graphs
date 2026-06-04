mod sealed {
    pub trait Sealed {}
}

pub trait ErrOr: sealed::Sealed {
    type Error;

    fn err_or<T>(self, value: T) -> Result<T, Self::Error>;
    fn err_or_else<T, F: FnOnce() -> T>(self, function: F) -> Result<T, Self::Error>;
}

impl<T> sealed::Sealed for Option<T> {}

impl<E> ErrOr for Option<E> {
    type Error = E;

    fn err_or<T>(self, value: T) -> Result<T, Self::Error> {
        self.map_or(Ok(value), Err)
    }

    fn err_or_else<T, F: FnOnce() -> T>(self, function: F) -> Result<T, Self::Error> {
        self.map_or_else(|| Ok(function()), Err)
    }
}

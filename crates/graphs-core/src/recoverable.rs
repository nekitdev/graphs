use core::{error::Error, fmt};

#[doc(hidden)]
pub mod import {
    pub use core::result::Result;
}

pub struct RecoverableError<E, R> {
    pub error: E,
    pub value: R,
}

impl<E: fmt::Debug, R> fmt::Debug for RecoverableError<E, R> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error().fmt(formatter)
    }
}

impl<E: fmt::Display, R> fmt::Display for RecoverableError<E, R> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error().fmt(formatter)
    }
}

impl<E: Error + 'static, R> Error for RecoverableError<E, R> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.error())
    }
}

impl<E, R> RecoverableError<E, R> {
    pub const fn new(error: E, value: R) -> Self {
        Self { error, value }
    }

    pub const fn error(&self) -> &E {
        &self.error
    }

    pub const fn error_mut(&mut self) -> &mut E {
        &mut self.error
    }

    pub const fn value(&self) -> &R {
        &self.value
    }

    pub const fn value_mut(&mut self) -> &mut R {
        &mut self.value
    }

    pub fn into_value(self) -> R {
        self.value
    }

    pub fn into_error(self) -> E {
        self.error
    }

    pub fn recover(self) -> (R, E) {
        (self.value, self.error)
    }
}

pub type Recoverable<T, E, R> = Result<T, RecoverableError<E, R>>;

#[macro_export]
macro_rules! recoverable_error {
    ($error: expr, $value: expr) => {
        $crate::recoverable::RecoverableError::new($error.into(), $value)
    };
}

#[macro_export]
macro_rules! recoverable {
    ($error: expr, $value: expr) => {
        $crate::recoverable::import::Result::Err($crate::recoverable_error!($error, $value))
    };
}

#[macro_export]
macro_rules! recoverable_return {
    ($result: expr, $value: expr) => {
        match $result {
            $crate::recoverable::import::Result::Ok(value) => value,
            $crate::recoverable::import::Result::Err(error) => {
                return $crate::recoverable!(error, $value);
            }
        }
    };
}

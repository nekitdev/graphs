use thiserror::Error;

#[doc(hidden)]
pub mod import {
    pub use core::result::Result;
}

#[derive(Debug, Error)]
#[error("{error} ({value})")]
pub struct Recoverable<E, R> {
    pub error: E,
    pub value: R,
}

impl<E, R> Recoverable<E, R> {
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

pub type RecoverableResult<T, E, R = T> = Result<T, Recoverable<E, R>>;

#[macro_export]
macro_rules! recoverable_error {
    ($error: expr, $value: expr) => {
        $crate::recoverable::Recoverable::new($error.into(), $value)
    };
}

#[macro_export]
macro_rules! recoverable_result {
    ($error: expr, $value: expr) => {
        $crate::recoverable::import::Result::Err($crate::recoverable_error!($error, $value))
    };
}

//! Calling methods on types by (mutable) reference.
//!
//! This module provides the [`By`] trait which allows for ergonomic chaining of method calls.

pub trait By {
    fn by_ref(&self) -> &Self {
        self
    }

    fn by_mut(&mut self) -> &mut Self {
        self
    }
}

impl<T: ?Sized> By for T {}

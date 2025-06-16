//! Freezing values.

use core::ops::Deref;

/// Represents frozen values.
///
/// Frozen values are used to prevent modifications to the underlying data.
/// This is achieved by holding *mutable* reference to `T` and giving *immutable* references
/// when required.
///
/// [`Frozen`] implements [`Deref`] to `T` along with [`AsRef<T>`].
///
/// This type is created by the [`Freeze`] trait, which exists to improve ergonomics.
pub struct Frozen<'f, T: ?Sized> {
    value: &'f mut T,
}

impl<'f, T: ?Sized> Frozen<'f, T> {
    /// Constructs [`Self`] from the given *mutable* reference to `T`.
    pub const fn new(value: &'f mut T) -> Self {
        Self { value }
    }
}

impl<T: ?Sized> AsRef<T> for Frozen<'_, T> {
    fn as_ref(&self) -> &T {
        self.value
    }
}

impl<T: ?Sized> Deref for Frozen<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

/// Represents types that can create [`Frozen`] values.
///
/// This trait is implemented for any `T` without requiring [`Sized`].
pub trait Freeze {
    /// Freezes [`Self`]. See [`Frozen`] for more details.
    fn freeze(&mut self) -> Frozen<'_, Self>;
}

impl<T: ?Sized> Freeze for T {
    fn freeze(&mut self) -> Frozen<'_, Self> {
        Frozen::new(self)
    }
}

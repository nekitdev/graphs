use crate::frozen::Frozen;

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

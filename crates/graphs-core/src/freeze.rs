use crate::{base::Base, frozen::Frozen};

/// Represents graphs that can be frozen.
///
/// This trait is implemented for any [`Base`] graph.
pub trait Freeze: Base {
    /// Freezes [`Self`]. See [`Frozen`] for more details.
    fn freeze(&mut self) -> Frozen<'_, Self> {
        Frozen::new(self)
    }
}

impl<G: Base + ?Sized> Freeze for G {}

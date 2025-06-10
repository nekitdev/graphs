//! Graph node and edge identifiers.

use core::hash::Hash;

/// Represents identifiers of nodes and edges in graphs.
pub trait Id: Copy + Ord + Hash + Default {
    /// The limiting value of the identifier.
    ///
    /// This is used to represent non-existing nodes or edges.
    const LIMIT: Self;

    /// Constructs [`Self`] from the given [`usize`] index.
    fn of(index: usize) -> Self;

    /// Converts the identifier into corresponding [`usize`] index.
    fn index(self) -> usize;

    /// Checks if the identifier is the limit.
    ///
    /// The default implementation simply compares [`Self`] with [`LIMIT`].
    ///
    /// [`LIMIT`]: Self::LIMIT
    fn is_limit(self) -> bool {
        self == Self::LIMIT
    }
}

impl Id for usize {
    const LIMIT: Self = Self::MAX;

    fn of(value: usize) -> Self {
        value
    }

    fn index(self) -> usize {
        self
    }
}

/// The default type used for identifiers in graphs.
pub type DefaultId = usize;

/// Represents types that contain identifiers.
pub trait Linked {
    /// The associated type for the identifier.
    type Id: Id;

    /// Returns the identifier of the item.
    fn id(&self) -> Self::Id;
}

impl<T: Linked + ?Sized> Linked for &T {
    type Id = T::Id;

    fn id(&self) -> Self::Id {
        (*self).id()
    }
}

impl<T: Linked + ?Sized> Linked for &mut T {
    type Id = T::Id;

    fn id(&self) -> Self::Id {
        (**self).id()
    }
}

//! Creating graphs.

use crate::capacity::Capacities;

/// Represents graphs that can be created.
pub trait Create {
    /// Constructs [`Self`].
    fn new() -> Self;

    /// Counstructs [`Self`] with the given [`Capacities`].
    fn with_capacity(capacities: Capacities) -> Self;
}

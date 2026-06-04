//! Creating graphs.

use crate::{base::Base, capacity::Capacities};

/// Represents graphs that can be created.
pub trait Create: Base {
    /// Constructs empty [`Self`].
    fn empty() -> Self;

    /// Counstructs [`Self`] with the given [`Capacities`].
    fn with_capacity(capacities: Capacities) -> Self;
}

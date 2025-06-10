//! Edge directions in graphs.

/// Represents edge directions, either *outgoing* or *incoming*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    /// *Outgoing* edge is directed *from* the context node.
    Outgoing,
    /// *Incoming* edge is directed *to* the context node.
    Incoming,
}

pub use Direction::{Incoming, Outgoing};

impl Direction {
    /// Checks whether the direction is [`Outgoing`].
    pub const fn is_outgoing(self) -> bool {
        matches!(self, Self::Outgoing)
    }

    /// Checks whether the direction is [`Incoming`].
    pub const fn is_incoming(self) -> bool {
        matches!(self, Self::Incoming)
    }

    /// Reverses the direction of the edge.
    ///
    /// [`Outgoing`] becomes [`Incoming`], and vice versa.
    #[must_use = "this method returns the reversed direction, without modifying the original"]
    pub const fn reversed(self) -> Self {
        match self {
            Self::Outgoing => Self::Incoming,
            Self::Incoming => Self::Outgoing,
        }
    }

    /// The count of all possible directions.
    pub const COUNT: usize = 2;

    /// The array of all possible directions.
    pub const ARRAY: [Self; Self::COUNT] = [Self::Outgoing, Self::Incoming];
}

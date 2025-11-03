//! Edge directions in graphs.

pub const OUTGOING: u8 = 0;
pub const INCOMING: u8 = 1;

/// Represents edge directions, either *outgoing* or *incoming*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Direction {
    /// *Outgoing* edge is directed *from* the context node.
    Outgoing = OUTGOING,
    /// *Incoming* edge is directed *to* the context node.
    Incoming = INCOMING,
}

pub use Direction::{Incoming, Outgoing};

impl Direction {
    /// Checks whether the direction is [`Outgoing`].
    #[must_use]
    pub const fn is_outgoing(self) -> bool {
        matches!(self, Self::Outgoing)
    }

    /// Checks whether the direction is [`Incoming`].
    #[must_use]
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
    pub const ALL: [Self; Self::COUNT] = [Self::Outgoing, Self::Incoming];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Directional<T> {
    pub direction: Direction,
    pub value: T,
}

impl<T> Directional<T> {
    pub const fn new(direction: Direction, value: T) -> Self {
        Self { direction, value }
    }

    pub const fn outgoing(value: T) -> Self {
        Self::new(Outgoing, value)
    }

    pub const fn incoming(value: T) -> Self {
        Self::new(Incoming, value)
    }

    pub const fn reverse(&mut self) {
        self.direction = self.direction.reversed();
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithDirection<T> {
    pub value: T,
    pub direction: Direction,
}

impl<T> WithDirection<T> {
    pub const fn new(value: T, direction: Direction) -> Self {
        Self { value, direction }
    }

    pub const fn outgoing(value: T) -> Self {
        Self::new(value, Outgoing)
    }

    pub const fn incoming(value: T) -> Self {
        Self::new(value, Incoming)
    }

    pub fn from_parts((value, direction): (T, Direction)) -> Self {
        Self::new(value, direction)
    }

    pub fn into_parts(self) -> (T, Direction) {
        (self.value, self.direction)
    }
}

impl<T> From<(T, Direction)> for WithDirection<T> {
    fn from(parts: (T, Direction)) -> Self {
        Self::from_parts(parts)
    }
}

impl<T> From<WithDirection<T>> for (T, Direction) {
    fn from(with_direction: WithDirection<T>) -> Self {
        with_direction.into_parts()
    }
}

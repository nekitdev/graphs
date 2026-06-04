pub const OUTGOING: &str = "outgoing";
pub const INCOMING: &str = "incoming";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Direction {
    Outgoing = 0,
    Incoming = 1,
}

impl fmt::Display for Direction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.static_str())
    }
}

use core::fmt;

pub use Direction::{Incoming, Outgoing};

impl Direction {
    pub const fn static_str(self) -> &'static str {
        match self {
            Self::Outgoing => OUTGOING,
            Self::Incoming => INCOMING,
        }
    }

    pub const fn is_outgoing(self) -> bool {
        matches!(self, Self::Outgoing)
    }

    pub const fn is_incoming(self) -> bool {
        matches!(self, Self::Incoming)
    }

    pub const fn reversed(self) -> Self {
        match self {
            Self::Outgoing => Self::Incoming,
            Self::Incoming => Self::Outgoing,
        }
    }

    pub const COUNT: usize = 2;

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
}

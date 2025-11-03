use crate::direction::{INCOMING, OUTGOING};

pub const ALL: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Select<T> {
    pub item: T,
    pub selection: Selection,
}

impl<T> Select<T> {
    pub const fn new(item: T, selection: Selection) -> Self {
        Self { item, selection }
    }

    pub const fn outgoing(item: T) -> Self {
        Self::new(item, Selection::Outgoing)
    }

    pub const fn incoming(item: T) -> Self {
        Self::new(item, Selection::Incoming)
    }

    pub const fn all(item: T) -> Self {
        Self::new(item, Selection::All)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum Selection {
    Outgoing = OUTGOING,
    Incoming = INCOMING,
    #[default]
    All = ALL,
}

impl Selection {
    /// Checks whether the selection is [`Outgoing`].
    #[must_use]
    pub const fn is_outgoing(self) -> bool {
        matches!(self, Self::Outgoing)
    }

    /// Checks whether the selection is [`Incoming`].
    #[must_use]
    pub const fn is_incoming(self) -> bool {
        matches!(self, Self::Incoming)
    }

    /// Checks whether the selection is [`All`].
    #[must_use]
    pub const fn is_all(self) -> bool {
        matches!(self, Self::All)
    }

    /// Reverses the direction of the edge.
    ///
    /// [`Outgoing`] becomes [`Incoming`], and vice versa.
    #[must_use = "this method returns the reversed selection, without modifying the original"]
    pub const fn reversed(self) -> Self {
        match self {
            Self::Outgoing => Self::Incoming,
            Self::Incoming => Self::Outgoing,
            Self::All => Self::All,
        }
    }
}

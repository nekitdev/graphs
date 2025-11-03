#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Class {
    Isolated,
    Source,
    Sink,
    #[default]
    General,
}

pub use Class::{General, Isolated, Sink, Source};

impl Class {
    #[must_use]
    pub const fn compute(outgoing: bool, incoming: bool) -> Self {
        match (outgoing, incoming) {
            (false, false) => Self::Isolated,
            (false, true) => Self::Sink,
            (true, false) => Self::Source,
            (true, true) => Self::General,
        }
    }

    #[must_use]
    pub const fn is_isolated(self) -> bool {
        matches!(self, Self::Isolated)
    }

    #[must_use]
    pub const fn is_source(self) -> bool {
        matches!(self, Self::Source)
    }

    #[must_use]
    pub const fn is_sink(self) -> bool {
        matches!(self, Self::Sink)
    }

    #[must_use]
    pub const fn is_general(self) -> bool {
        matches!(self, Self::General)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Degrees {
    pub outgoing: usize,
    pub incoming: usize,
}

impl Degrees {
    #[must_use]
    pub const fn new(outgoing: usize, incoming: usize) -> Self {
        Self { outgoing, incoming }
    }

    #[must_use]
    pub const fn total(self) -> usize {
        self.outgoing + self.incoming
    }

    #[must_use]
    pub const fn class(self) -> Class {
        Class::compute(self.outgoing != 0, self.incoming != 0)
    }
}

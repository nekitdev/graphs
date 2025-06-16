#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Class {
    Isolated,
    Source,
    Sink,
    #[default]
    Regular,
}

pub use Class::{Isolated, Regular, Sink, Source};

impl Class {
    pub const fn compute(outgoing: bool, incoming: bool) -> Self {
        match (outgoing, incoming) {
            (false, false) => Isolated,
            (false, true) => Sink,
            (true, false) => Source,
            (true, true) => Regular,
        }
    }

    pub const fn is_isolated(self) -> bool {
        matches!(self, Self::Isolated)
    }

    pub const fn is_source(self) -> bool {
        matches!(self, Self::Source)
    }

    pub const fn is_sink(self) -> bool {
        matches!(self, Self::Sink)
    }

    pub const fn is_regular(self) -> bool {
        matches!(self, Self::Regular)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Degrees {
    pub outgoing: usize,
    pub incoming: usize,
}

impl Degrees {
    pub const fn new(outgoing: usize, incoming: usize) -> Self {
        Self { outgoing, incoming }
    }

    pub const fn total(self) -> usize {
        self.outgoing + self.incoming
    }

    pub const fn class(self) -> Class {
        Class::compute(self.outgoing != 0, self.incoming != 0)
    }
}

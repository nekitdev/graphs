//! Graph kinds.

use core::fmt;

use crate::markers::{Marker, Private, StaticStr};

mod sealed {
    pub trait Sealed {}
}

/// Represents graph kinds, either [`Directed`] or [`Undirected`].
pub trait Kind: Marker + sealed::Sealed {
    /// Indicates whether the graph is *directed*.
    const DIRECTED: bool;

    type Inverse: Kind<Inverse = Self>;
}

/// The `directed` literal.
pub const DIRECTED: &str = "directed";

/// The `undirected` literal.
pub const UNDIRECTED: &str = "undirected";

/// Represents *directed* graph kind.
pub struct Directed {
    private: Private,
}

/// Represents *undirected* graph kind.
pub struct Undirected {
    private: Private,
}

impl Marker for Directed {
    const NAME: StaticStr = stringify!(Directed);

    fn display(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(DIRECTED)
    }
}

impl Marker for Undirected {
    const NAME: StaticStr = stringify!(Undirected);

    fn display(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(UNDIRECTED)
    }
}

impl sealed::Sealed for Directed {}
impl sealed::Sealed for Undirected {}

impl Kind for Directed {
    const DIRECTED: bool = true;

    type Inverse = Undirected;
}

impl Kind for Undirected {
    const DIRECTED: bool = false;

    type Inverse = Directed;
}

pub type DefaultKind = Directed;

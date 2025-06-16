//! Graph kinds.

use core::{fmt, marker::PhantomData};

mod sealed {
    pub trait Sealed {}
}

/// Represents graphs with specified [`Kind`].
pub trait Kinded {
    /// Associated type representing the *kind* of the graph.
    type Kind: Kind + ?Sized;
}

impl<G: Kinded + ?Sized> Kinded for &G {
    type Kind = G::Kind;
}

impl<G: Kinded + ?Sized> Kinded for &mut G {
    type Kind = G::Kind;
}

/// Represents graph kinds, either [`Directed`] or [`Undirected`].
pub trait Kind: sealed::Sealed {
    /// Indicates whether the graph is *directed*.
    const IS_DIRECTED: bool;

    /// Returns [`Output`] that can be used for formatting [`Self`] kind.
    fn output() -> Output<Self> {
        Output::new()
    }
}

/// Represents *directed* graph kind.
pub struct Directed {
    private: PhantomData<()>,
}

/// Represents *undirected* graph kind.
pub struct Undirected {
    private: PhantomData<()>,
}

impl sealed::Sealed for Directed {}
impl sealed::Sealed for Undirected {}

impl Kind for Directed {
    const IS_DIRECTED: bool = true;
}

impl Kind for Undirected {
    const IS_DIRECTED: bool = false;
}

/// Wraps [`Kind`] to provide formatting implementations.
pub struct Output<K: Kind + ?Sized> {
    kind: PhantomData<K>,
}

impl<K: Kind + ?Sized> Output<K> {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self { kind: PhantomData }
    }
}

/// The [`Directed`] name.
pub const DIRECTED_NAME: &str = "Directed";

/// The [`Undirected`] name.
pub const UNDIRECTED_NAME: &str = "Undirected";

/// The `directed` literal.
pub const DIRECTED: &str = "directed";

/// The `undirected` literal.
pub const UNDIRECTED: &str = "undirected";

impl<K: Kind + ?Sized> fmt::Debug for Output<K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if K::IS_DIRECTED {
            formatter.write_str(DIRECTED_NAME)
        } else {
            formatter.write_str(UNDIRECTED_NAME)
        }
    }
}

impl<K: Kind + ?Sized> fmt::Display for Output<K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if K::IS_DIRECTED {
            formatter.write_str(DIRECTED)
        } else {
            formatter.write_str(UNDIRECTED)
        }
    }
}

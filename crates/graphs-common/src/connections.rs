use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};
use std::mem::swap;

use graphs_core::{
    connections::{Connection, Parts},
    id::NodeTypeId,
    kinds::{DefaultKind, Kind},
    markers::MarkerOutput,
};

use crate::internals::debug_struct;

/// Represents *kinded* connections, that is, connections *generic* over their [`Kind`].
///
/// In many cases, it is advisable for connections to have concrete kinds, though this
/// type exists to cover the edge cases that can occur in graph implementations.
pub struct Kinded<N: NodeTypeId, K: Kind = DefaultKind> {
    /// Node *one* of the connection, which is the *source* item for [`Directed`] connections.
    pub one: N,

    /// Node *two* of the connection, which is the *target* item for [`Directed`] connections.
    pub two: N,

    kind: PhantomData<K>,
}

impl<N: NodeTypeId, K: Kind> Clone for Kinded<N, K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N: NodeTypeId, K: Kind> Copy for Kinded<N, K> {}

impl<N: NodeTypeId, K: Kind> PartialEq for Kinded<N, K> {
    fn eq(&self, other: &Self) -> bool {
        self.one == other.one && self.two == other.two
    }
}

impl<N: NodeTypeId, K: Kind> Eq for Kinded<N, K> {}

impl<N: NodeTypeId, K: Kind> PartialOrd for Kinded<N, K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: NodeTypeId, K: Kind> Ord for Kinded<N, K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.one
            .cmp(&other.one)
            .then_with(|| self.two.cmp(&other.two))
    }
}

impl<N: NodeTypeId, K: Kind> Hash for Kinded<N, K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.one.hash(state);
        self.two.hash(state);
    }
}

impl<N: NodeTypeId, K: Kind> fmt::Debug for Kinded<N, K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_struct!(
            formatter => Kinded {
                one: self.one,
                two: self.two,
                kind: K::output(),
            }
        )
        .finish()
    }
}

impl<N: NodeTypeId, K: Kind> fmt::Display for Kinded<N, K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if K::DIRECTED {
            write!(
                formatter,
                "{source} -> {target}",
                source = self.one,
                target = self.two
            )
        } else {
            write!(formatter, "{one} <-> {two}", one = self.one, two = self.two)
        }
    }
}

impl<N: NodeTypeId, K: Kind> Kinded<N, K> {
    pub const fn new(one: N, two: N) -> Self {
        Self {
            one,
            two,
            kind: PhantomData,
        }
    }

    pub const fn reverse(&mut self) {
        swap(&mut self.one, &mut self.two);
    }
}

impl<N: NodeTypeId, K: Kind> Connection for Kinded<N, K> {
    type NodeId = N;
    type Kind = K;

    type Inverse = Kinded<N, K::Inverse>;

    fn connecting(one: Self::NodeId, two: Self::NodeId) -> Self {
        Self::new(one, two)
    }

    fn parts(&self) -> Parts<Self> {
        (self.one, self.two)
    }

    fn inverse(self) -> Self::Inverse {
        Self::Inverse::new(self.one, self.two)
    }
}

use graphs_core::{
    index::{DefaultUntypedIndex, UntypedIndex},
    kinds::{DefaultKind, Kind},
    sentinel::Sentinel,
};

use graphs_common::{
    connections::Kinded,
    index::{EdgeIndex, NodeIndex},
    next::Next as NextEdge,
};

use crate::references::{EdgeRef, NodeRef};

pub type Next<I = DefaultUntypedIndex> = NextEdge<EdgeIndex<I>>;

pub struct Node<T, I: UntypedIndex = DefaultUntypedIndex> {
    pub value: T,
    pub next: Next<I>,
}

pub type NodeSlice<'n, T, I = DefaultUntypedIndex> = &'n [Node<T, I>];
pub type NodeMutSlice<'n, T, I = DefaultUntypedIndex> = &'n mut [Node<T, I>];

impl<T, I: UntypedIndex> Node<T, I> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            next: Next::SENTINEL,
        }
    }

    pub const fn value(&self) -> &T {
        &self.value
    }

    pub const fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn get(self) -> T {
        self.value
    }

    pub const fn as_ref_with(&self, index: NodeIndex<I>) -> NodeRef<'_, T, I> {
        NodeRef::new(index, self.value())
    }

    pub const fn reverse(&mut self) {
        self.next.reverse();
    }

    pub const fn reset(&mut self) {
        let _ = self.next.reset();
    }
}

pub type Connection<I = DefaultUntypedIndex, K = DefaultKind> = Kinded<NodeIndex<I>, K>;

pub struct Edge<T, I: UntypedIndex = DefaultUntypedIndex, K: Kind = DefaultKind> {
    pub value: T,
    pub connection: Connection<I, K>,
    pub next: Next<I>,
}

pub type EdgeSlice<'e, T, I = DefaultUntypedIndex, K = DefaultKind> = &'e [Edge<T, I, K>];
pub type EdgeMutSlice<'e, T, I = DefaultUntypedIndex, K = DefaultKind> = &'e mut [Edge<T, I, K>];

impl<T, I: UntypedIndex, K: Kind> Edge<T, I, K> {
    pub const fn new(value: T, connection: Connection<I, K>) -> Self {
        Self {
            value,
            connection,
            next: Next::SENTINEL,
        }
    }

    pub const fn connecting(value: T, one: NodeIndex<I>, two: NodeIndex<I>) -> Self {
        Self::new(value, Connection::new(one, two))
    }

    pub const fn value(&self) -> &T {
        &self.value
    }

    pub const fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn get(self) -> T {
        self.value
    }

    pub const fn as_ref_with(&self, index: EdgeIndex<I>) -> EdgeRef<'_, T, I, K> {
        EdgeRef::new(index, self.connection(), self.value())
    }

    pub const fn connection(&self) -> Connection<I, K> {
        self.connection
    }

    pub const fn reverse(&mut self) {
        self.connection.reverse();
        self.next.reverse();
    }
}

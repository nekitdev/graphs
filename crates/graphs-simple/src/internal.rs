#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use graphs_core::{
    connection::IndexConnection,
    keys::{DefaultUntypedIndex, EdgeIndex, NodeIndex, UntypedIndex},
    next::IndexNext,
};

pub type InternalNext<I = DefaultUntypedIndex> = IndexNext<EdgeIndex<I>>;

pub struct Node<T, I: UntypedIndex = DefaultUntypedIndex> {
    pub value: T,
    pub next: InternalNext<I>,
}

impl<T, I: UntypedIndex> Node<T, I> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            next: InternalNext::limit(),
        }
    }

    pub const fn get_ref(&self) -> &T {
        &self.value
    }

    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn get(self) -> T {
        self.value
    }

    pub const fn reverse(&mut self) {
        self.next.reverse();
    }

    pub const fn reset(&mut self) {
        self.next.reset();
    }
}

pub type InternalConnection<I = DefaultUntypedIndex> = IndexConnection<NodeIndex<I>>;

pub struct Edge<T, I: UntypedIndex = DefaultUntypedIndex> {
    pub connection: InternalConnection<I>,
    pub value: T,
    pub next: InternalNext<I>,
}

impl<T, I: UntypedIndex> Edge<T, I> {
    pub const fn new(connection: InternalConnection<I>, value: T) -> Self {
        Self {
            connection,
            value,
            next: InternalNext::limit(),
        }
    }

    pub const fn connecting(source: NodeIndex<I>, target: NodeIndex<I>, value: T) -> Self {
        Self::new(InternalConnection::new(source, target), value)
    }

    pub const fn get_ref(&self) -> &T {
        &self.value
    }

    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub const fn reverse(&mut self) {
        self.connection.reverse();
        self.next.reverse();
    }

    pub fn get(self) -> T {
        self.value
    }
}

pub type Nodes<T, I = DefaultUntypedIndex> = [Node<T, I>];
pub type Edges<T, I = DefaultUntypedIndex> = [Edge<T, I>];

pub type NodeVec<T, I = DefaultUntypedIndex> = Vec<Node<T, I>>;
pub type EdgeVec<T, I = DefaultUntypedIndex> = Vec<Edge<T, I>>;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::vec::Vec;
    } else if #[cfg(feature = "alloc")] {
        use alloc::vec::Vec;
    } else {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

use graphs_core::{
    connection::{Connection, Connector},
    id::{DefaultId, Id},
};

use crate::{index::NodeIndex, next::Next};

pub struct Node<T, I: Id = DefaultId> {
    pub value: T,
    pub next: Next<I>,
}

impl<T, I: Id> Node<T, I> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            next: Next::limit(),
        }
    }

    pub const fn get(&self) -> &T {
        &self.value
    }

    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn take(self) -> T {
        self.value
    }
}

pub type Connecting<I = DefaultId> = Connection<NodeIndex<I>>;

pub struct Edge<T, I: Id = DefaultId> {
    pub connecting: Connecting<I>,
    pub value: T,
    pub next: Next<I>,
}

impl<T, I: Id> Edge<T, I> {
    pub const fn new(connecting: Connecting<I>, value: T) -> Self {
        Self {
            connecting,
            value,
            next: Next::limit(),
        }
    }

    pub const fn connecting(source: NodeIndex<I>, target: NodeIndex<I>, value: T) -> Self {
        Self::new(Connecting::new(source, target), value)
    }

    pub const fn get(&self) -> &T {
        &self.value
    }

    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn take(self) -> T {
        self.value
    }
}

impl<T, I: Id> Connector for Edge<T, I> {
    type Id = NodeIndex<I>;

    fn connection(&self) -> Connection<Self::Id> {
        self.connecting.copy()
    }
}

pub type Nodes<T, I = DefaultId> = Vec<Node<T, I>>;
pub type Edges<T, I = DefaultId> = Vec<Edge<T, I>>;

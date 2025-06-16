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

use graphs_core::id::DefaultNodeId;

pub struct Node<T, I: IndexId = DefaultId> {
    pub value: T,
    pub next: EdgeNext<I>,
}

impl<T, I: IndexId> Node<T, I> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            next: EdgeNext::limit(),
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

pub struct Edge<T, I: IndexId = DefaultId> {
    pub connection: NodeConnection<I>,
    pub value: T,
    pub next: EdgeNext<I>,
}

impl<T, I: IndexId> Edge<T, I> {
    pub const fn new(connection: NodeConnection<I>, value: T) -> Self {
        Self {
            connection,
            value,
            next: EdgeNext::limit(),
        }
    }

    pub const fn connecting(source: NodeId<I>, target: NodeId<I>, value: T) -> Self {
        Self::new(NodeConnection::new(source, target), value)
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

pub type Nodes<T, I = DefaultId> = Vec<Node<T, I>>;
pub type Edges<T, I = DefaultId> = Vec<Edge<T, I>>;

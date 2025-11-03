use graphs_core::{
    connections::Kinded,
    index::{DefaultUntypedIndex, EdgeIndex, NodeIndex, UntypedIndex},
    kinds::{DefaultKind, Kind},
    limit::Limited,
    next,
};

pub type Next<I = DefaultUntypedIndex> = next::Next<EdgeIndex<I>>;

pub struct Node<T, I: UntypedIndex = DefaultUntypedIndex> {
    pub value: T,
    pub next: Next<I>,
}

impl<T, K: UntypedIndex> Node<T, K> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            next: Next::LIMIT,
        }
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

impl<T, I: UntypedIndex, K: Kind> Edge<T, I, K> {
    pub const fn new(value: T, connection: Connection<I, K>) -> Self {
        Self {
            value,
            connection,
            next: Next::LIMIT,
        }
    }

    pub const fn reverse(&mut self) {
        self.connection.reverse();
        self.next.reverse();
    }
}

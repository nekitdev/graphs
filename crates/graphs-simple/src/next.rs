use core::mem::{replace, swap};

use graphs_core::{
    direction::{Direction, Incoming, Outgoing},
    id::{DefaultId, EdgeId, Id},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Next<I: Id = DefaultId> {
    pub outgoing: EdgeId<I>,
    pub incoming: EdgeId<I>,
}

impl<I: Id> Next<I> {
    pub const fn new(outgoing: EdgeId<I>, incoming: EdgeId<I>) -> Self {
        Self { outgoing, incoming }
    }

    pub const fn same(index: EdgeId<I>) -> Self {
        Self::new(index, index)
    }

    pub const fn limit() -> Self {
        Self::same(EdgeId::LIMIT)
    }

    pub const fn directed(&self, direction: Direction) -> EdgeId<I> {
        match direction {
            Outgoing => self.outgoing,
            Incoming => self.incoming,
        }
    }

    pub const fn replace_in(&mut self, direction: Direction, index: EdgeId<I>) -> EdgeId<I> {
        match direction {
            Outgoing => self.replace_outgoing(index),
            Incoming => self.replace_incoming(index),
        }
    }

    pub const fn replace_outgoing(&mut self, outgoing: EdgeId<I>) -> EdgeId<I> {
        replace(&mut self.outgoing, outgoing)
    }

    pub const fn replace_incoming(&mut self, incoming: EdgeId<I>) -> EdgeId<I> {
        replace(&mut self.incoming, incoming)
    }

    pub const fn replace(&mut self, other: Self) -> Self {
        replace(self, other)
    }

    pub const fn replace_same(&mut self, index: EdgeId<I>) -> Self {
        replace(self, Self::same(index))
    }

    pub const fn reverse(&mut self) {
        swap(&mut self.outgoing, &mut self.incoming);
    }
}

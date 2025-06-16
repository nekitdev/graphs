use core::mem::replace;

use crate::{
    direction::{Direction, Incoming, Outgoing},
    id::{DefaultEdgeId, EdgeTypeId},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Next<E: EdgeTypeId = DefaultEdgeId> {
    pub outgoing: E,
    pub incoming: E,
}

impl<E: EdgeTypeId> Next<E> {
    pub const fn new(outgoing: E, incoming: E) -> Self {
        Self { outgoing, incoming }
    }

    pub const fn same(value: E) -> Self {
        Self::new(value, value)
    }

    pub const fn limit() -> Self {
        Self::same(E::LIMIT)
    }

    pub const fn directed(&self, direction: Direction) -> E {
        match direction {
            Outgoing => self.outgoing,
            Incoming => self.incoming,
        }
    }

    pub const fn replace_directed(&mut self, direction: Direction, value: E) -> E {
        match direction {
            Outgoing => self.replace_outgoing(value),
            Incoming => self.replace_incoming(value),
        }
    }

    pub const fn replace_outgoing(&mut self, outgoing: E) -> E {
        replace(&mut self.outgoing, outgoing)
    }

    pub const fn replace_incoming(&mut self, incoming: E) -> E {
        replace(&mut self.incoming, incoming)
    }

    pub const fn replace_same(&mut self, value: E) -> Self {
        replace(self, Self::same(value))
    }
}

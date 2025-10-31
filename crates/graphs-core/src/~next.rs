use core::mem::{replace, swap};

use crate::{
    direction::{Direction, Incoming, Outgoing},
    id::{DefaultEdgeId, EdgeTypeId},
    keys::{DefaultEdgeIndex, EdgeTypeIndex},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Next<E: EdgeTypeId = DefaultEdgeId> {
    pub outgoing: E,
    pub incoming: E,
}

pub type DefaultNext = Next<DefaultEdgeId>;

impl<E: EdgeTypeId> Default for Next<E> {
    fn default() -> Self {
        Self::limit()
    }
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

    #[allow(clippy::mem_replace_with_default)] // needed for `const`
    pub const fn reset(&mut self) -> Self {
        replace(self, Self::limit())
    }

    pub const fn reverse(&mut self) {
        swap(&mut self.outgoing, &mut self.incoming);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexNext<E: EdgeTypeIndex = DefaultEdgeIndex> {
    pub outgoing: E,
    pub incoming: E,
}

pub type DefaultIndexNext = IndexNext<DefaultEdgeIndex>;

impl<E: EdgeTypeIndex> From<Next<E>> for IndexNext<E> {
    fn from(next: Next<E>) -> Self {
        Self::new(next.outgoing, next.incoming)
    }
}

impl<E: EdgeTypeIndex> From<IndexNext<E>> for Next<E> {
    fn from(next: IndexNext<E>) -> Self {
        Self::new(next.outgoing, next.incoming)
    }
}

impl<E: EdgeTypeIndex> Default for IndexNext<E> {
    fn default() -> Self {
        Self::limit()
    }
}

impl<E: EdgeTypeIndex> IndexNext<E> {
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

    pub const fn reset(&mut self) -> Self {
        replace(self, Self::limit())
    }

    pub const fn reverse(&mut self) {
        swap(&mut self.outgoing, &mut self.incoming);
    }
}

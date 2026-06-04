use core::mem::{replace, swap};

use graphs_core::{
    direction::Direction::{self, Incoming, Outgoing},
    id::EdgeTypeId,
    sentinel::Sentinel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Next<E: EdgeTypeId> {
    pub outgoing: E,
    pub incoming: E,
}

impl<E: EdgeTypeId> Sentinel for Next<E> {
    const SENTINEL: Self = Self::new(E::SENTINEL, E::SENTINEL);

    fn is_sentinel(&self) -> bool {
        self.outgoing.is_sentinel() && self.incoming.is_sentinel()
    }
}

impl<E: EdgeTypeId> Next<E> {
    pub const fn new(outgoing: E, incoming: E) -> Self {
        Self { outgoing, incoming }
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

    // pub const fn directed(&self, direction: Direction) -> &T {
    //     match direction {
    //         Outgoing => &self.outgoing,
    //         Incoming => &self.incoming,
    //     }
    // }

    // pub const fn directed_mut(&mut self, direction: Direction) -> &mut T {
    //     match direction {
    //         Outgoing => &mut self.outgoing,
    //         Incoming => &mut self.incoming,
    //     }
    // }

    // pub const fn replace_directed(&mut self, direction: Direction, value: T) -> T {
    //     match direction {
    //         Outgoing => self.replace_outgoing(value),
    //         Incoming => self.replace_incoming(value),
    //     }
    // }

    pub const fn replace_outgoing(&mut self, value: E) -> E {
        replace(&mut self.outgoing, value)
    }

    pub const fn replace_incoming(&mut self, value: E) -> E {
        replace(&mut self.incoming, value)
    }

    pub const fn reverse(&mut self) {
        swap(&mut self.outgoing, &mut self.incoming);
    }

    #[must_use]
    pub const fn reset(&mut self) -> Self {
        replace(self, Self::SENTINEL)
    }
}

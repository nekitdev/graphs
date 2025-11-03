use core::mem::{replace, swap};

use crate::{
    direction::{Direction, Incoming, Outgoing},
    limit::Limited,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Next<T: Limited> {
    pub outgoing: T,
    pub incoming: T,
}

impl<T: Limited> Limited for Next<T> {
    const LIMIT: Self = Self::new(T::LIMIT, T::LIMIT);
}

impl<T: Limited> Next<T> {
    pub const fn new(outgoing: T, incoming: T) -> Self {
        Self { outgoing, incoming }
    }

    pub const fn directed(&self, direction: Direction) -> &T {
        match direction {
            Outgoing => &self.outgoing,
            Incoming => &self.incoming,
        }
    }

    pub const fn directed_mut(&mut self, direction: Direction) -> &mut T {
        match direction {
            Outgoing => &mut self.outgoing,
            Incoming => &mut self.incoming,
        }
    }

    pub const fn replace_directed(&mut self, direction: Direction, value: T) -> T {
        match direction {
            Outgoing => self.replace_outgoing(value),
            Incoming => self.replace_incoming(value),
        }
    }

    pub const fn replace_outgoing(&mut self, outgoing: T) -> T {
        replace(&mut self.outgoing, outgoing)
    }

    pub const fn replace_incoming(&mut self, incoming: T) -> T {
        replace(&mut self.incoming, incoming)
    }

    pub const fn reverse(&mut self) {
        swap(&mut self.outgoing, &mut self.incoming);
    }

    #[must_use]
    pub const fn reset(&mut self) -> Self {
        replace(self, Self::LIMIT)
    }
}

use crate::{base::Base, connections::Connection, data::Data};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge<T, C: Connection> {
    pub connection: C,
    pub value: T,
}

impl<T, C: Connection> From<(C::NodeId, C::NodeId, T)> for Edge<T, C> {
    fn from((one, two, value): (C::NodeId, C::NodeId, T)) -> Self {
        Self::connecting(one, two, value)
    }
}

impl<T: Default, C: Connection> From<C> for Edge<T, C> {
    fn from(connection: C) -> Self {
        Self::new_default(connection)
    }
}

impl<T: Default, C: Connection> From<(C::NodeId, C::NodeId)> for Edge<T, C> {
    fn from((one, two): (C::NodeId, C::NodeId)) -> Self {
        Self::connecting_default(one, two)
    }
}

impl<T, C: Connection> Edge<T, C> {
    pub const fn new(connection: C, value: T) -> Self {
        Self { connection, value }
    }

    pub const fn value(&self) -> &T {
        &self.value
    }

    pub const fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn connection(&self) -> C {
        self.connection
    }

    pub fn get(self) -> (C, T) {
        (self.connection, self.value)
    }

    pub fn connecting(one: C::NodeId, two: C::NodeId, value: T) -> Self {
        Self::new(C::connecting(one, two), value)
    }
}

impl<T: Default, C: Connection> Edge<T, C> {
    pub fn new_default(connection: C) -> Self {
        Self::new(connection, T::default())
    }

    pub fn connecting_default(one: C::NodeId, two: C::NodeId) -> Self {
        Self::connecting(one, two, T::default())
    }
}

pub type EdgeIn<G> = Edge<<G as Data>::EdgeValue, <G as Base>::Connection>;

use core::error::Error;

use crate::{
    base::Base,
    build::TryAddEdges,
    data::Data,
    edges::{Edge, EdgeIn},
    id::EdgeTypeId,
    recoverable::Recoverable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Taken<T, E: EdgeTypeId> {
    pub value: T,
    pub edge: E,
}

impl<T, E: EdgeTypeId> Taken<T, E> {
    pub const fn new(value: T, edge: E) -> Self {
        Self { value, edge }
    }
}

pub type TakenIn<G> = Taken<<G as Data>::EdgeValue, <G as Base>::EdgeId>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Updated<T, E: EdgeTypeId> {
    NotTaken(E),
    Taken(Taken<T, E>),
}

pub type UpdatedIn<G> = Updated<<G as Data>::EdgeValue, <G as Base>::EdgeId>;

pub type RecoverableUpdated<G> =
    Recoverable<UpdatedIn<G>, <G as TryUpdate>::Error, <G as Data>::EdgeValue>;

pub trait TryUpdate: TryAddEdges {
    type Error: Error;

    fn try_update(&mut self, edge: EdgeIn<Self>) -> RecoverableUpdated<Self>;

    fn try_update_with(
        &mut self,
        connection: Self::Connection,
        value: Self::EdgeValue,
    ) -> RecoverableUpdated<Self> {
        self.try_update(Edge::new(connection, value))
    }

    fn try_update_default(&mut self, connection: Self::Connection) -> RecoverableUpdated<Self>
    where
        Self::EdgeValue: Default,
    {
        self.try_update(Edge::new_default(connection))
    }

    fn try_update_connecting(
        &mut self,
        one: Self::NodeId,
        two: Self::NodeId,
        value: Self::EdgeValue,
    ) -> RecoverableUpdated<Self> {
        self.try_update(Edge::connecting(one, two, value))
    }

    fn try_update_connecting_default(
        &mut self,
        one: Self::NodeId,
        two: Self::NodeId,
    ) -> RecoverableUpdated<Self>
    where
        Self::EdgeValue: Default,
    {
        self.try_update(Edge::connecting_default(one, two))
    }
}

impl<G: TryUpdate + ?Sized> TryUpdate for &mut G {
    type Error = <G as TryUpdate>::Error;

    fn try_update(&mut self, edge: EdgeIn<Self>) -> RecoverableUpdated<Self> {
        (*self).try_update(edge)
    }
}

pub const UPDATE: &str = "failed to update edge";
pub const UPDATE_WITH: &str = "failed to update edge with connection and value";
pub const UPDATE_DEFAULT: &str = "failed to update edge with connection and default value";
pub const UPDATE_CONNECTING: &str = "failed to update edge connecting nodes with value";
pub const UPDATE_CONNECTING_DEFAULT: &str =
    "failed to update edge connecting nodes with default value";

pub trait Update: TryUpdate {
    fn update(&mut self, edge: EdgeIn<Self>) -> UpdatedIn<Self> {
        self.try_update(edge).expect(UPDATE)
    }

    fn update_with(
        &mut self,
        connection: Self::Connection,
        value: Self::EdgeValue,
    ) -> UpdatedIn<Self> {
        self.try_update_with(connection, value).expect(UPDATE_WITH)
    }

    fn update_default(&mut self, connection: Self::Connection) -> UpdatedIn<Self>
    where
        Self::EdgeValue: Default,
    {
        self.try_update_default(connection).expect(UPDATE_DEFAULT)
    }

    fn update_connecting(
        &mut self,
        one: Self::NodeId,
        two: Self::NodeId,
        value: Self::EdgeValue,
    ) -> UpdatedIn<Self> {
        self.try_update_connecting(one, two, value)
            .expect(UPDATE_CONNECTING)
    }

    fn update_connecting_default(&mut self, one: Self::NodeId, two: Self::NodeId) -> UpdatedIn<Self>
    where
        Self::EdgeValue: Default,
    {
        self.try_update_connecting_default(one, two)
            .expect(UPDATE_CONNECTING_DEFAULT)
    }
}

impl<G: TryUpdate + ?Sized> Update for G {}

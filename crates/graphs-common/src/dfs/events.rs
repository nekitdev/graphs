use core::fmt;

use graphs_core::{base::Base, connections::Connection, id::NodeTypeId};

use crate::{id::DefaultNodeId, time::Time};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timed<N: NodeTypeId = DefaultNodeId> {
    pub node: N,
    pub time: Time,
}

impl<N: NodeTypeId> fmt::Display for Timed<N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{node}@{time}",
            node = self.node(),
            time = self.time()
        )
    }
}

impl<N: NodeTypeId> Timed<N> {
    pub const fn new(node: N, time: Time) -> Self {
        Self { node, time }
    }

    pub const fn node(&self) -> N {
        self.node
    }

    pub const fn time(&self) -> Time {
        self.time
    }
}

pub type TimedOf<C> = Timed<<C as Connection>::NodeId>;

pub enum Event<C: Connection> {
    Discover(TimedOf<C>),
    Tree(C),
    Back(C),
    Cross(C),
    Forward(C),
    Finish(TimedOf<C>),
}

pub type EventIn<G> = Event<<G as Base>::Connection>;

impl<C: Connection> Event<C> {
    pub const fn discover(node: C::NodeId, time: Time) -> Self {
        Self::Discover(Timed::new(node, time))
    }

    pub fn tree(node: C::NodeId, neighbor: C::NodeId) -> Self {
        Self::Tree(C::connecting(node, neighbor))
    }

    pub fn back(node: C::NodeId, neighbor: C::NodeId) -> Self {
        Self::Back(C::connecting(node, neighbor))
    }

    pub fn cross(node: C::NodeId, neighbor: C::NodeId) -> Self {
        Self::Cross(C::connecting(node, neighbor))
    }

    pub fn forward(node: C::NodeId, neighbor: C::NodeId) -> Self {
        Self::Forward(C::connecting(node, neighbor))
    }

    pub const fn finish(node: C::NodeId, time: Time) -> Self {
        Self::Finish(Timed::new(node, time))
    }
}

pub enum SimpleEvent<C: Connection> {
    Discover(TimedOf<C>),
    Tree(C),
    Back(C),
    CrossOrForward(C),
    Finish(TimedOf<C>),
}

pub type SimpleEventIn<G> = SimpleEvent<<G as Base>::Connection>;

impl<C: Connection> SimpleEvent<C> {
    pub const fn discover(node: C::NodeId, time: Time) -> Self {
        Self::Discover(Timed::new(node, time))
    }

    pub fn tree(node: C::NodeId, neighbor: C::NodeId) -> Self {
        Self::Tree(C::connecting(node, neighbor))
    }

    pub fn back(node: C::NodeId, neighbor: C::NodeId) -> Self {
        Self::Back(C::connecting(node, neighbor))
    }

    pub fn cross_or_forward(node: C::NodeId, neighbor: C::NodeId) -> Self {
        Self::CrossOrForward(C::connecting(node, neighbor))
    }

    pub const fn finish(node: C::NodeId, time: Time) -> Self {
        Self::Finish(Timed::new(node, time))
    }
}

impl<C: Connection> From<Event<C>> for SimpleEvent<C> {
    fn from(event: Event<C>) -> Self {
        match event {
            Event::Discover(timed) => Self::Discover(timed),
            Event::Tree(connection) => Self::Tree(connection),
            Event::Back(connection) => Self::Back(connection),
            Event::Cross(connection) => Self::CrossOrForward(connection),
            Event::Forward(connection) => Self::CrossOrForward(connection),
            Event::Finish(timed) => Self::Finish(timed),
        }
    }
}

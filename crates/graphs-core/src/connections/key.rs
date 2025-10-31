use core::mem::swap;

use crate::{
    connections::Connection,
    id::{DefaultNodeId, EdgeType, Id, NodeTypeId},
    kinds,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Directed<N: NodeTypeId = DefaultNodeId> {
    pub source: N,
    pub target: N,
}

pub type DefaultDirected = Directed<DefaultNodeId>;

impl<N: NodeTypeId> Default for Directed<N> {
    fn default() -> Self {
        Self::limit()
    }
}

impl<N: NodeTypeId> Id for Directed<N> {
    type Type = EdgeType;

    const LIMIT: Self = Self::limit();
}

impl<N: NodeTypeId> Connection for Directed<N> {
    type Id = N;

    type Kind = kinds::Directed;

    type Inverse = Undirected<N>;

    fn connecting(one: Self::Id, two: Self::Id) -> Self {
        Self::new(one, two)
    }
}

impl<N: NodeTypeId> Directed<N> {
    pub const fn new(source: N, target: N) -> Self {
        Self { source, target }
    }

    pub const fn limit() -> Self {
        Self::new(N::LIMIT, N::LIMIT)
    }

    pub const fn reverse(&mut self) {
        swap(&mut self.source, &mut self.target);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Undirected<N: NodeTypeId = DefaultNodeId> {
    pub one: N,
    pub two: N,
}

pub type DefaultUndirected = Undirected<DefaultNodeId>;

impl<N: NodeTypeId> Default for Undirected<N> {
    fn default() -> Self {
        Self::limit()
    }
}

impl<N: NodeTypeId> Id for Undirected<N> {
    type Type = EdgeType;

    const LIMIT: Self = Self::limit();
}

impl<N: NodeTypeId> Connection for Undirected<N> {
    type Id = N;

    type Kind = kinds::Undirected;

    type Inverse = Directed<N>;

    fn connecting(one: Self::Id, two: Self::Id) -> Self {
        Self::new(one, two)
    }
}

impl<N: NodeTypeId> Undirected<N> {
    pub const fn new(one: N, two: N) -> Self {
        Self { one, two }
    }

    pub const fn limit() -> Self {
        Self::new(N::LIMIT, N::LIMIT)
    }
}

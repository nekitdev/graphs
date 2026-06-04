use trait_aliases::trait_aliases;

use crate::{
    connections::{Connection, DirectedConnection, UndirectedConnection},
    data::Data,
    id::{EdgeTypeId, NodeTypeId},
    identifiers::{EdgeIdentifiers, NodeIdentifiers},
};

pub trait NodeRef {
    type NodeId: NodeTypeId;
    type Value;

    fn id(&self) -> Self::NodeId;
    fn value(&self) -> &Self::Value;
}

pub trait EdgeRef {
    type EdgeId: EdgeTypeId;

    type Connection: Connection;

    type Value;

    fn id(&self) -> Self::EdgeId;

    fn connection(&self) -> Self::Connection;

    fn value(&self) -> &Self::Value;
}

trait_aliases! {
    #[trait_alias(E)]
    pub trait DirectedEdgeRef = EdgeRef<Connection: DirectedConnection>;

    #[trait_alias(E)]
    pub trait UndirectedEdgeRef = EdgeRef<Connection: UndirectedConnection>;
}

trait_aliases! {
    #[trait_alias(N)]
    pub trait NodeRefIn<G: Data + ?Sized> = NodeRef<NodeId = G::NodeId, Value = G::NodeValue>;

    #[trait_alias(E)]
    pub trait EdgeRefIn<G: Data + ?Sized> = EdgeRef<
            EdgeId = G::EdgeId,
            Connection = G::Connection,
            Value = G::EdgeValue,
        >;
}

pub trait NodeReferences: Data + NodeIdentifiers {
    type NodeRef<'g>: NodeRefIn<Self>
    where
        Self: 'g;

    type NodeRefIterator<'g>: Iterator<Item = Self::NodeRef<'g>>
    where
        Self: 'g;

    fn node_references(&self) -> Self::NodeRefIterator<'_>;
}

impl<G: NodeReferences + ?Sized> NodeReferences for &G {
    type NodeRef<'g>
        = G::NodeRef<'g>
    where
        Self: 'g;

    type NodeRefIterator<'g>
        = G::NodeRefIterator<'g>
    where
        Self: 'g;

    fn node_references(&self) -> Self::NodeRefIterator<'_> {
        (*self).node_references()
    }
}

impl<G: NodeReferences + ?Sized> NodeReferences for &mut G {
    type NodeRef<'g>
        = G::NodeRef<'g>
    where
        Self: 'g;

    type NodeRefIterator<'g>
        = G::NodeRefIterator<'g>
    where
        Self: 'g;

    fn node_references(&self) -> Self::NodeRefIterator<'_> {
        (**self).node_references()
    }
}

pub trait EdgeReferences: Data + EdgeIdentifiers {
    type EdgeRef<'g>: EdgeRefIn<Self>
    where
        Self: 'g;

    type EdgeRefIterator<'g>: Iterator<Item = Self::EdgeRef<'g>>
    where
        Self: 'g;

    fn edge_references(&self) -> Self::EdgeRefIterator<'_>;
}

impl<G: EdgeReferences + ?Sized> EdgeReferences for &G {
    type EdgeRef<'g>
        = G::EdgeRef<'g>
    where
        Self: 'g;

    type EdgeRefIterator<'g>
        = G::EdgeRefIterator<'g>
    where
        Self: 'g;

    fn edge_references(&self) -> Self::EdgeRefIterator<'_> {
        (*self).edge_references()
    }
}

impl<G: EdgeReferences + ?Sized> EdgeReferences for &mut G {
    type EdgeRef<'g>
        = G::EdgeRef<'g>
    where
        Self: 'g;

    type EdgeRefIterator<'g>
        = G::EdgeRefIterator<'g>
    where
        Self: 'g;

    fn edge_references(&self) -> Self::EdgeRefIterator<'_> {
        (**self).edge_references()
    }
}

trait_aliases! {
    #[trait_alias(G)]
    pub trait References = NodeReferences + EdgeReferences;
}

pub type UnnamedNodeRef<'a, N, T = ()> = (N, &'a T);

pub struct NamedNodeRef<'a, N: NodeTypeId, T = ()> {
    pub id: N,
    pub value: &'a T,
}

impl<N: NodeTypeId> From<N> for NamedNodeRef<'_, N> {
    fn from(id: N) -> Self {
        Self::empty(id)
    }
}

impl<'a, N: NodeTypeId, T> From<UnnamedNodeRef<'a, N, T>> for NamedNodeRef<'a, N, T> {
    fn from((id, value): UnnamedNodeRef<'a, N, T>) -> Self {
        Self::new(id, value)
    }
}

impl<N: NodeTypeId> NamedNodeRef<'_, N> {
    pub const fn empty(id: N) -> Self {
        Self::new(id, &())
    }
}

impl<'a, N: NodeTypeId, T> NamedNodeRef<'a, N, T> {
    pub const fn new(id: N, value: &'a T) -> Self {
        Self { id, value }
    }

    pub const fn get(&self) -> &'a T {
        self.value
    }
}

impl<N: NodeTypeId, T> NodeRef for UnnamedNodeRef<'_, N, T> {
    type NodeId = N;
    type Value = T;

    fn id(&self) -> Self::NodeId {
        self.0
    }

    fn value(&self) -> &Self::Value {
        self.1
    }
}

impl<N: NodeTypeId, T> NodeRef for NamedNodeRef<'_, N, T> {
    type NodeId = N;
    type Value = T;

    fn id(&self) -> Self::NodeId {
        self.id
    }

    fn value(&self) -> &Self::Value {
        self.get()
    }
}

pub type UnnamedEdgeRef<'a, E, C, T = ()> = (E, C, &'a T);

pub struct NamedEdgeRef<'a, E: EdgeTypeId, C: Connection, T = ()> {
    pub id: E,
    pub connection: C,
    pub value: &'a T,
}

impl<E: EdgeTypeId, C: Connection> From<(E, C)> for NamedEdgeRef<'_, E, C> {
    fn from((id, connection): (E, C)) -> Self {
        Self::empty(id, connection)
    }
}

impl<'a, E: EdgeTypeId, C: Connection, T> From<UnnamedEdgeRef<'a, E, C, T>>
    for NamedEdgeRef<'a, E, C, T>
{
    fn from((id, connection, value): UnnamedEdgeRef<'a, E, C, T>) -> Self {
        Self::new(id, connection, value)
    }
}

impl<E: EdgeTypeId, C: Connection> NamedEdgeRef<'_, E, C> {
    pub const fn empty(id: E, connection: C) -> Self {
        Self::new(id, connection, &())
    }
}

impl<'a, E: EdgeTypeId, C: Connection, T> NamedEdgeRef<'a, E, C, T> {
    pub const fn new(id: E, connection: C, value: &'a T) -> Self {
        Self {
            id,
            connection,
            value,
        }
    }

    pub const fn get(&self) -> &'a T {
        self.value
    }
}

impl<E: EdgeTypeId, C: Connection, T> NamedEdgeRef<'_, E, C, T> {
    pub const fn get_connection(&self) -> &C {
        &self.connection
    }
}

impl<E: EdgeTypeId, C: Connection, T> EdgeRef for UnnamedEdgeRef<'_, E, C, T> {
    type EdgeId = E;
    type Connection = C;
    type Value = T;

    fn id(&self) -> Self::EdgeId {
        self.0
    }

    fn connection(&self) -> Self::Connection {
        self.1
    }

    fn value(&self) -> &Self::Value {
        self.2
    }
}

impl<E: EdgeTypeId, C: Connection, T> EdgeRef for NamedEdgeRef<'_, E, C, T> {
    type EdgeId = E;
    type Connection = C;
    type Value = T;

    fn id(&self) -> Self::EdgeId {
        self.id
    }

    fn connection(&self) -> Self::Connection {
        self.connection
    }

    fn value(&self) -> &Self::Value {
        self.get()
    }
}

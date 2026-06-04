use core::{iter::Enumerate, slice::Iter};

use graphs_common::index::{EdgeIndex, NodeIndex};

use graphs_core::{
    index::{DefaultUntypedIndex, Index, UntypedIndex},
    kinds::{DefaultKind, Directed, Kind, Undirected},
    references::{NamedEdgeRef, NamedNodeRef},
};

use crate::parts::{Connection, Edge, Node};

pub type NodeRef<'a, N, I = DefaultUntypedIndex> = NamedNodeRef<'a, NodeIndex<I>, N>;
pub type EdgeRef<'a, E, I = DefaultUntypedIndex, K = DefaultKind> =
    NamedEdgeRef<'a, EdgeIndex<I>, Connection<I, K>, E>;

pub type DirectedEdgeRef<'a, E, I = DefaultUntypedIndex> = EdgeRef<'a, E, I, Directed>;
pub type UndirectedEdgeRef<'a, E, I = DefaultUntypedIndex> = EdgeRef<'a, E, I, Undirected>;

pub struct NodeRefIterator<'a, N, I: UntypedIndex = DefaultUntypedIndex> {
    iterator: Enumerate<Iter<'a, Node<N, I>>>,
}

impl<'a, N, I: UntypedIndex> NodeRefIterator<'a, N, I> {
    pub(crate) fn new(iterator: Enumerate<Iter<'a, Node<N, I>>>) -> Self {
        Self { iterator }
    }
}

impl<'a, I: UntypedIndex, N> Iterator for NodeRefIterator<'a, N, I> {
    type Item = NodeRef<'a, N, I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|(index, node)| node.as_ref_with(NodeIndex::of(index)))
    }
}

pub struct EdgeRefIterator<'a, E, I: UntypedIndex = DefaultUntypedIndex, K: Kind = DefaultKind> {
    iterator: Enumerate<Iter<'a, Edge<E, I, K>>>,
}

impl<'a, E, I: UntypedIndex, K: Kind> EdgeRefIterator<'a, E, I, K> {
    pub(crate) fn new(iterator: Enumerate<Iter<'a, Edge<E, I, K>>>) -> Self {
        Self { iterator }
    }
}

impl<'a, I: UntypedIndex, E, K: Kind> Iterator for EdgeRefIterator<'a, E, I, K> {
    type Item = EdgeRef<'a, E, I, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|(index, edge)| edge.as_ref_with(EdgeIndex::of(index)))
    }
}

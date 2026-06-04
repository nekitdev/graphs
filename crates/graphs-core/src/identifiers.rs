use trait_aliases::trait_aliases;

use crate::base::Base;

pub trait NodeIdentifiers: Base {
    type NodeIdIterator<'g>: Iterator<Item = Self::NodeId>
    where
        Self: 'g;

    fn node_identifiers(&self) -> Self::NodeIdIterator<'_>;
}

impl<G: NodeIdentifiers + ?Sized> NodeIdentifiers for &G {
    type NodeIdIterator<'g>
        = G::NodeIdIterator<'g>
    where
        Self: 'g;

    fn node_identifiers(&self) -> Self::NodeIdIterator<'_> {
        (*self).node_identifiers()
    }
}

impl<G: NodeIdentifiers + ?Sized> NodeIdentifiers for &mut G {
    type NodeIdIterator<'i>
        = G::NodeIdIterator<'i>
    where
        Self: 'i;

    fn node_identifiers(&self) -> Self::NodeIdIterator<'_> {
        (**self).node_identifiers()
    }
}

pub trait EdgeIdentifiers: Base {
    type EdgeIdIterator<'g>: Iterator<Item = Self::EdgeId>
    where
        Self: 'g;

    fn edge_identifiers(&self) -> Self::EdgeIdIterator<'_>;
}

impl<G: EdgeIdentifiers + ?Sized> EdgeIdentifiers for &G {
    type EdgeIdIterator<'g>
        = G::EdgeIdIterator<'g>
    where
        Self: 'g;

    fn edge_identifiers(&self) -> Self::EdgeIdIterator<'_> {
        (*self).edge_identifiers()
    }
}

impl<G: EdgeIdentifiers + ?Sized> EdgeIdentifiers for &mut G {
    type EdgeIdIterator<'g>
        = G::EdgeIdIterator<'g>
    where
        Self: 'g;

    fn edge_identifiers(&self) -> Self::EdgeIdIterator<'_> {
        (**self).edge_identifiers()
    }
}

trait_aliases! {
    #[trait_alias(G)]
    pub trait Identifiers = NodeIdentifiers + EdgeIdentifiers;
}

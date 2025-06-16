use crate::base::Base;

pub trait NodeIdentifiers: Base {
    type Identifiers<'i>: Iterator<Item = Self::NodeId>
    where
        Self: 'i;

    fn node_identifiers(&self) -> Self::Identifiers<'_>;
}

impl<G: NodeIdentifiers + ?Sized> NodeIdentifiers for &G {
    type Identifiers<'i>
        = G::Identifiers<'i>
    where
        Self: 'i;

    fn node_identifiers(&self) -> Self::Identifiers<'_> {
        (*self).node_identifiers()
    }
}

impl<G: NodeIdentifiers + ?Sized> NodeIdentifiers for &mut G {
    type Identifiers<'i>
        = G::Identifiers<'i>
    where
        Self: 'i;

    fn node_identifiers(&self) -> Self::Identifiers<'_> {
        (**self).node_identifiers()
    }
}

pub trait EdgeIdentifiers: Base {
    type Identifiers<'i>: Iterator<Item = Self::NodeId>
    where
        Self: 'i;

    fn edge_identifiers(&self) -> Self::Identifiers<'_>;
}

impl<G: EdgeIdentifiers + ?Sized> EdgeIdentifiers for &G {
    type Identifiers<'i>
        = G::Identifiers<'i>
    where
        Self: 'i;

    fn edge_identifiers(&self) -> Self::Identifiers<'_> {
        (*self).edge_identifiers()
    }
}

impl<G: EdgeIdentifiers + ?Sized> EdgeIdentifiers for &mut G {
    type Identifiers<'i>
        = G::Identifiers<'i>
    where
        Self: 'i;

    fn edge_identifiers(&self) -> Self::Identifiers<'_> {
        (**self).edge_identifiers()
    }
}

pub trait Identifiers: NodeIdentifiers + EdgeIdentifiers {}

impl<G: NodeIdentifiers + EdgeIdentifiers> Identifiers for G {}

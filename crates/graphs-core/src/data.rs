//! Traits for graphs that contain data.

use crate::{
    base::Base,
    items::{IdOf, ValueMutOf, ValueRefOf},
    map_item,
};

/// Represents graphs that contain data.
pub trait Data: Base {
    /// The associated type of node values.
    type NodeValue;

    /// The associated type of edge values.
    type EdgeValue;
}

impl<G: Data + ?Sized> Data for &G {
    type NodeValue = G::NodeValue;
    type EdgeValue = G::EdgeValue;
}

impl<G: Data + ?Sized> Data for &mut G {
    type NodeValue = G::NodeValue;
    type EdgeValue = G::EdgeValue;
}

/// Represents graphs that can map their node and edge identifiers to values by reference.
pub trait DataRef: Data {
    /// Returns the node value corresponding to the given identifier, if any.
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue>;

    /// Returns the edge value corresponding to the given identifier, if any.
    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue>;

    fn value(&self, self_id: IdOf<Self>) -> Option<ValueRefOf<'_, Self>> {
        map_item!(
            self_id,
            node_id => self.node_value(node_id),
            edge_id => self.edge_value(edge_id),
        )
        .factor_none()
    }
}

impl<G: DataRef + ?Sized> DataRef for &G {
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
        (*self).node_value(id)
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        (*self).edge_value(id)
    }
}

impl<G: DataRef + ?Sized> DataRef for &mut G {
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
        (**self).node_value(id)
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        (**self).edge_value(id)
    }
}

/// Represents graphs that can map their node and edge identifiers to values by mutable reference.
pub trait DataMut: DataRef {
    /// Returns the mutable node value corresponding to the given identifier, if any.
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue>;

    /// Returns the mutable edge value corresponding to the given identifier, if any.
    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue>;

    fn value_mut(&mut self, self_id: IdOf<Self>) -> Option<ValueMutOf<'_, Self>> {
        map_item!(
            self_id,
            node_id => self.node_value_mut(node_id),
            edge_id => self.edge_value_mut(edge_id),
        )
        .factor_none()
    }
}

impl<G: DataMut + ?Sized> DataMut for &mut G {
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
        (*self).node_value_mut(id)
    }

    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
        (*self).edge_value_mut(id)
    }
}

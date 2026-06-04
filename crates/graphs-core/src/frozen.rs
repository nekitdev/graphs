//! Frozen graphs.

use core::ops::Deref;

use crate::{
    attached::Edges,
    base::Base,
    capacity::{EdgeCapacity, NodeCapacity},
    cardinality::{Order, Size},
    data::{Data, DataMut, DataRef},
    identifiers::{EdgeIdentifiers, NodeIdentifiers},
    indexed::{EdgeCompact, EdgeIndexed, NodeCompact, NodeIndexed},
    references::{EdgeReferences, NodeReferences},
    visit::Visit,
};

/// Represents frozen graphs.
///
/// This structure is used to prevent modifications to the *structure* of the underlying graph,
/// but it allows any *immutable* access along with structure-preserving *mutable* access.
///
/// This is achieved by holding the *mutable* reference to `G` and giving *immutable* references
/// when required.
///
/// [`Frozen<G>`] implements [`Deref`] to `G` along with [`AsRef<G>`].
///
/// This type is created by the [`Freeze`] trait, which exists to improve ergonomics.
///
/// [`Freeze`]: crate::freeze::Freeze
pub struct Frozen<'f, G: Base + ?Sized> {
    value: &'f mut G,
}

impl<'f, G: Base + ?Sized> Frozen<'f, G> {
    /// Constructs [`Self`] from the given *mutable* reference to `G`.
    pub const fn new(value: &'f mut G) -> Self {
        Self { value }
    }

    /// Returns *immutable* references to the underlying graph.
    #[must_use]
    pub const fn get(&self) -> &G {
        self.value
    }

    /// Unfreezes the graph via consuming `Self` and retuning the contained mutable reference.
    #[must_use]
    pub const fn unfreeze(self) -> &'f mut G {
        self.value
    }

    // NOTE: this function is private to maintain the structure-preserving invariant
    pub(crate) const fn get_mut(&mut self) -> &mut G {
        self.value
    }
}

impl<G: Base + ?Sized> AsRef<G> for Frozen<'_, G> {
    fn as_ref(&self) -> &G {
        self.get()
    }
}

impl<G: Base + ?Sized> Deref for Frozen<'_, G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<G: Base + ?Sized> Base for Frozen<'_, G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;

    type Connection = G::Connection;

    type Kind = G::Kind;
    type Type = G::Type;
    type Loop = G::Loop;
}

impl<G: Data + ?Sized> Data for Frozen<'_, G> {
    type NodeValue = G::NodeValue;
    type EdgeValue = G::EdgeValue;
}

impl<G: DataRef + ?Sized> DataRef for Frozen<'_, G> {
    fn node_value(&self, id: Self::NodeId) -> Option<&Self::NodeValue> {
        self.get().node_value(id)
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        self.get().edge_value(id)
    }
}

impl<G: DataMut + ?Sized> DataMut for Frozen<'_, G> {
    fn node_value_mut(&mut self, id: Self::NodeId) -> Option<&mut Self::NodeValue> {
        self.get_mut().node_value_mut(id)
    }

    fn edge_value_mut(&mut self, id: Self::EdgeId) -> Option<&mut Self::EdgeValue> {
        self.get_mut().edge_value_mut(id)
    }
}

impl<G: Visit + ?Sized> Visit for Frozen<'_, G> {
    type Visitor = G::Visitor;

    fn build_visitor(&self) -> Self::Visitor {
        self.get().build_visitor()
    }

    fn reset_visitor(&self, visitor: &mut Self::Visitor) {
        self.get().reset_visitor(visitor);
    }
}

impl<G: NodeCapacity + Base + ?Sized> NodeCapacity for Frozen<'_, G> {
    fn node_capacity(&self) -> usize {
        self.get().node_capacity()
    }
}

impl<G: EdgeCapacity + Base + ?Sized> EdgeCapacity for Frozen<'_, G> {
    fn edge_capacity(&self) -> usize {
        self.get().edge_capacity()
    }
}

impl<G: Edges + ?Sized> Edges for Frozen<'_, G> {
    type EdgeIterator<'g>
        = G::EdgeIterator<'g>
    where
        Self: 'g;

    fn edges(&self, node: Self::NodeId) -> Self::EdgeIterator<'_> {
        self.get().edges(node)
    }
}

// TODO: `DirectedEdges`

impl<G: NodeIdentifiers + ?Sized> NodeIdentifiers for Frozen<'_, G> {
    type NodeIdIterator<'g>
        = G::NodeIdIterator<'g>
    where
        Self: 'g;

    fn node_identifiers(&self) -> Self::NodeIdIterator<'_> {
        self.get().node_identifiers()
    }
}

impl<G: EdgeIdentifiers + ?Sized> EdgeIdentifiers for Frozen<'_, G> {
    type EdgeIdIterator<'g>
        = G::EdgeIdIterator<'g>
    where
        Self: 'g;

    fn edge_identifiers(&self) -> Self::EdgeIdIterator<'_> {
        self.get().edge_identifiers()
    }
}

impl<G: NodeReferences + ?Sized> NodeReferences for Frozen<'_, G> {
    type NodeRef<'g>
        = G::NodeRef<'g>
    where
        Self: 'g;

    type NodeRefIterator<'g>
        = G::NodeRefIterator<'g>
    where
        Self: 'g;

    fn node_references(&self) -> Self::NodeRefIterator<'_> {
        self.get().node_references()
    }
}

impl<G: EdgeReferences + ?Sized> EdgeReferences for Frozen<'_, G> {
    type EdgeRef<'g>
        = G::EdgeRef<'g>
    where
        Self: 'g;

    type EdgeRefIterator<'g>
        = G::EdgeRefIterator<'g>
    where
        Self: 'g;

    fn edge_references(&self) -> Self::EdgeRefIterator<'_> {
        self.get().edge_references()
    }
}

impl<G: Order + Base + ?Sized> Order for Frozen<'_, G> {
    fn order(&self) -> usize {
        self.get().order()
    }
}

impl<G: Size + Base + ?Sized> Size for Frozen<'_, G> {
    fn size(&self) -> usize {
        self.get().size()
    }
}

impl<G: NodeIndexed + ?Sized> NodeIndexed for Frozen<'_, G> {
    fn node_bound(&self) -> usize {
        self.get().node_bound()
    }

    fn try_node_index(&self, id: Self::NodeId) -> Option<usize> {
        self.get().try_node_index(id)
    }

    fn try_node_id(&self, index: usize) -> Option<Self::NodeId> {
        self.get().try_node_id(index)
    }

    fn node_index(&self, id: Self::NodeId) -> usize {
        self.get().node_index(id)
    }

    fn node_id(&self, index: usize) -> Self::NodeId {
        self.get().node_id(index)
    }
}

impl<G: EdgeIndexed + ?Sized> EdgeIndexed for Frozen<'_, G> {
    fn edge_bound(&self) -> usize {
        self.get().edge_bound()
    }

    fn try_edge_index(&self, id: Self::EdgeId) -> Option<usize> {
        self.get().try_edge_index(id)
    }

    fn try_edge_id(&self, index: usize) -> Option<Self::EdgeId> {
        self.get().try_edge_id(index)
    }

    fn edge_index(&self, id: Self::EdgeId) -> usize {
        self.get().edge_index(id)
    }

    fn edge_id(&self, index: usize) -> Self::EdgeId {
        self.get().edge_id(index)
    }
}

impl<G: NodeCompact + ?Sized> NodeCompact for Frozen<'_, G> {}
impl<G: EdgeCompact + ?Sized> EdgeCompact for Frozen<'_, G> {}

// XXX: the following should never be implemented:
//
// - `Build`
// - `Clear` and `ClearEdges`
// - `Reverse`
//
// TODO: it is up for discussion whether `ReserveNodes` and `ReserveEdges` should be implemented

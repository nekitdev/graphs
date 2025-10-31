//! Frozen values.

use core::ops::Deref;

use crate::{
    base::Base,
    data::{Data, DataMut, DataRef},
    visit::Visit,
};

/// Represents frozen values.
///
/// Frozen values are used to prevent modifications to the underlying data.
/// This is achieved by holding *mutable* reference to `T` and giving *immutable* references
/// when required.
///
/// [`Frozen`] implements [`Deref`] to `T` along with [`AsRef<T>`].
///
/// This type is created by the [`Freeze`] trait, which exists to improve ergonomics.
///
/// [`Freeze`]: crate::freeze::Freeze
pub struct Frozen<'f, T: ?Sized> {
    value: &'f mut T,
}

impl<'f, T: ?Sized> Frozen<'f, T> {
    /// Constructs [`Self`] from the given *mutable* reference to `T`.
    pub const fn new(value: &'f mut T) -> Self {
        Self { value }
    }

    pub const fn get_ref(&self) -> &T {
        self.value
    }

    pub const fn get(self) -> &'f mut T {
        self.value
    }

    pub(crate) const fn get_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<T: ?Sized> AsRef<T> for Frozen<'_, T> {
    fn as_ref(&self) -> &T {
        self.get_ref()
    }
}

impl<T: ?Sized> Deref for Frozen<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_ref()
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
        self.get_ref().node_value(id)
    }

    fn edge_value(&self, id: Self::EdgeId) -> Option<&Self::EdgeValue> {
        self.get_ref().edge_value(id)
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
        self.get_ref().build_visitor()
    }

    fn reset_visitor(&self, visitor: &mut Self::Visitor) {
        self.get_ref().reset_visitor(visitor);
    }
}

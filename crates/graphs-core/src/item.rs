//! Simplified graph node and edge types.

use core::error::Error;

use crate::{data::Data, internal::failed};

/// Represents nodes in graphs.
///
/// Nodes are implicitly identified by the index of their appearance.
pub struct Node<T> {
    /// The value of the node.
    pub value: T,
}

impl<T> Node<T> {
    /// Constructs [`Self`] with the given value.
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns the value of the node behind immutable reference.
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Returns the value of the node behind mutable reference.
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Takes the value of the node, consuming it.
    pub fn take(self) -> T {
        self.value
    }
}

impl<T: Clone> Node<&T> {
    pub fn cloned(self) -> Node<T> {
        Node::new(self.take().clone())
    }
}

impl<T: Clone> Node<&mut T> {
    pub fn cloned(self) -> Node<T> {
        Node::new(self.take().clone())
    }
}

impl<T: Copy> Node<&T> {
    pub fn copied(self) -> Node<T> {
        Node::new(self.take().clone())
    }
}

impl<T: Copy> Node<&mut T> {
    pub fn copied(self) -> Node<T> {
        Node::new(self.take().clone())
    }
}

/// Represents edges in graphs.
///
/// The edges connect nodes via implicit indices.
pub struct Edge<T> {
    /// The source node index.
    pub source: usize,

    /// The target node index.
    pub target: usize,

    /// The value of the edge.
    pub value: T,
}

impl<T> Edge<T> {
    /// Constructs [`Self`].
    pub const fn new(source: usize, target: usize, value: T) -> Self {
        Self {
            source,
            target,
            value,
        }
    }

    /// Returns the value of the edge behind immutable reference.
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Returns the value of the edge behind mutable reference.
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Takes the value of the edge, consuming it.
    pub fn take(self) -> T {
        self.value
    }
}

impl<T: Clone> Edge<&T> {
    pub fn cloned(self) -> Edge<T> {
        Edge::new(self.source, self.target, self.take().clone())
    }
}

impl<T: Clone> Edge<&mut T> {
    pub fn cloned(self) -> Edge<T> {
        Edge::new(self.source, self.target, self.take().clone())
    }
}

impl<T: Copy> Edge<&T> {
    pub fn copied(self) -> Edge<T> {
        Edge::new(self.source, self.target, self.take().clone())
    }
}

impl<T: Copy> Edge<&mut T> {
    pub fn copied(self) -> Edge<T> {
        Edge::new(self.source, self.target, self.take().clone())
    }
}

/// Represents items that can be used to construct graphs.
pub enum Item<N, E> {
    /// The node item.
    Node(Node<N>),

    /// The edge item.
    Edge(Edge<E>),
}

impl<N: Clone, E: Clone> Item<&N, &E> {
    pub fn cloned(self) -> Item<N, E> {
        match self {
            Self::Node(node) => Item::Node(node.cloned()),
            Self::Edge(edge) => Item::Edge(edge.cloned()),
        }
    }
}

impl<N: Clone, E: Clone> Item<&mut N, &mut E> {
    pub fn cloned(self) -> Item<N, E> {
        match self {
            Self::Node(node) => Item::Node(node.cloned()),
            Self::Edge(edge) => Item::Edge(edge.cloned()),
        }
    }
}

impl<N: Copy, E: Copy> Item<&N, &E> {
    pub fn copied(self) -> Item<N, E> {
        match self {
            Self::Node(node) => Item::Node(node.copied()),
            Self::Edge(edge) => Item::Edge(edge.copied()),
        }
    }
}

impl<N: Copy, E: Copy> Item<&mut N, &mut E> {
    pub fn copied(self) -> Item<N, E> {
        match self {
            Self::Node(node) => Item::Node(node.copied()),
            Self::Edge(edge) => Item::Edge(edge.copied()),
        }
    }
}

/// Represents graphs that can be constructed from items (represented by [`Item`]).
pub trait FromItems: Data + Sized {
    /// The associated error type that is returned from [`try_from_items`].
    ///
    /// [`try_from_items`]: Self::try_from_items
    type FromItemsError: Error;

    /// Attempts to construct new graph from the given iterable of items.
    ///
    /// # Errors
    ///
    /// Returns [`FromItemsError`] if the graph could not be constructed.
    ///
    /// [`FromItemsError`]: Self::FromItemsError
    fn try_from_items<I>(iterable: I) -> Result<Self, Self::FromItemsError>
    where
        I: IntoIterator<Item = Item<Self::NodeValue, Self::EdgeValue>>;

    /// Panicking version of [`try_from_items`].
    ///
    /// # Panics
    ///
    /// Panics if the graph could not be constructed.
    ///
    /// [`try_from_items`]: Self::try_from_items
    fn from_items<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = Item<Self::NodeValue, Self::EdgeValue>>,
    {
        Self::try_from_items(iterable).expect(failed!(from_items))
    }
}

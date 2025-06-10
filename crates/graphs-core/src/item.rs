use core::error;

use cfg_if::cfg_if;
use thiserror::Error;

use crate::{build::Build, create::Create, internal::failed};

pub struct Node<T> {
    pub value: T,
}

impl<T> Node<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    pub const fn get(&self) -> &T {
        &self.value
    }

    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn take(self) -> T {
        self.value
    }
}

pub struct Edge<T> {
    pub source: usize,
    pub target: usize,
    pub value: T,
}

impl<T> Edge<T> {
    pub const fn new(source: usize, target: usize, value: T) -> Self {
        Self {
            source,
            target,
            value,
        }
    }

    pub const fn get(&self) -> &T {
        &self.value
    }

    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn take(self) -> T {
        self.value
    }
}

pub enum Item<N, E> {
    Node(Node<N>),
    Edge(Edge<E>),
}

pub trait FromItems: Data + Sized {
    type FromItemsError: error::Error;

    fn try_from_items<I>(iterable: I) -> Result<Self, Self::FromItemsError>
    where
        I: IntoIterator<Item = Item<Self::NodeValue, Self::EdgeValue>>;

    fn from_items<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = Item<Self::NodeValue, Self::EdgeValue>>,
    {
        Self::try_from_items(iterable).expect(failed!(from_items))
    }
}

#[derive(Debug, Error)]
pub enum Error<M, F> {
    #[error("node `{0}` is missing")]
    Missing(usize),
    #[error("call to `add_node` failed: {0}")]
    Node(M),
    #[error("call to `add_edge` failed: {0}")]
    Edge(F),
}

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::vec::Vec;
    } else if #[cfg(feature = "alloc")] {
        use alloc::vec::Vec;
    }
}

cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        pub fn try_from_items<G: Create + Build, I>(
            iterable: I,
        ) -> Result<G, Error<G::NodeError, G::EdgeError>>
        where
            I: IntoIterator<Item = Item<G::NodeValue, G::EdgeValue>>,
        {
            let mut graph = G::empty();
            let mut map = Vec::new();

            for item in iterable {
                match item {
                    Item::Node(node) => {
                        let id = graph.try_add_node(node.take()).map_err(Error::Node)?;

                        map.push(id);
                    }
                    Item::Edge(edge) => {
                        let source_index = edge.source;
                        let target_index = edge.target;

                        let Some(source_id) = map.get(source_index).copied() else {
                            return Err(Error::Missing(source_index));
                        };

                        let Some(target_id) = map.get(target_index).copied() else {
                            return Err(Error::Missing(target_index));
                        };

                        graph
                            .try_add_edge(source_id, target_id, edge.take())
                            .map_err(Error::Edge)?;
                    }
                }
            }

            Ok(graph)
        }

        pub fn from_items<G: Create + Build, I>(iterable: I) -> G
        where
            I: IntoIterator<Item = Item<G::NodeValue, G::EdgeValue>>,
        {
            try_from_items(iterable).expect(failed!(from_items))
        }
    }
}

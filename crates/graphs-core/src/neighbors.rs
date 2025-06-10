use crate::{base::Base, direction::Direction};

pub trait Neighbors: Base {
    type Iterator<'n>: Iterator<Item = Self::NodeId>
    where
        Self: 'n;

    fn neighbors(&self, start: Self::NodeId) -> Self::Iterator<'_>;
}

pub trait DirectedNeighbors: Neighbors {
    type DirectedIterator<'n>: Iterator<Item = Self::NodeId>
    where
        Self: 'n;

    fn directed_neighbors(
        &self,
        start: Self::NodeId,
        direction: Direction,
    ) -> Self::DirectedIterator<'_>;
}

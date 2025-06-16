use crate::adapters::{reversed::Reversed, undirected::Undirected};

pub trait Adapt {
    fn as_reversed(&self) -> Reversed<&Self> {
        Reversed::new(self)
    }

    fn as_reversed_mut(&mut self) -> Reversed<&mut Self> {
        Reversed::new(self)
    }

    fn into_reversed(self) -> Reversed<Self>
    where
        Self: Sized,
    {
        Reversed::new(self)
    }

    fn as_undirected(&self) -> Undirected<&Self> {
        Undirected::new(self)
    }

    fn as_undirected_mut(&mut self) -> Undirected<&mut Self> {
        Undirected::new(self)
    }

    fn into_undirected(self) -> Undirected<Self>
    where
        Self: Sized,
    {
        Undirected::new(self)
    }
}

impl<G: ?Sized> Adapt for G {}

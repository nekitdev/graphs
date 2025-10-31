//! Adapters for directed graphs.

use crate::base::Directed;

pub mod reversed;
pub mod undirected;

pub use reversed::Reversed;
pub use undirected::Undirected;

pub trait Adapters: Directed {
    fn reversed(self) -> Reversed<Self>
    where
        Self: Sized,
    {
        Reversed::new(self)
    }

    fn undirected(self) -> Undirected<Self>
    where
        Self: Sized,
    {
        Undirected::new(self)
    }
}

impl<G: Directed + ?Sized> Adapters for G {}

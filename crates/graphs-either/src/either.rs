use core::{
    ffi::CStr,
    fmt::Debug,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    both,
    copy::copy,
    either,
    internals::fail,
    macros::{edge_and_then, node_and_then},
    map_both, map_either,
};

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<N, E> {
    Node(N),
    Edge(E),
}

impl<N: Clone, E: Clone> Clone for Either<N, E> {
    fn clone(&self) -> Self {
        map_both!(self, inner => inner.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Node(self_node), Node(source_node)) => self_node.clone_from(source_node),
            (Edge(self_edge), Edge(source_edge)) => self_edge.clone_from(source_edge),
            (either, different_source) => *either = different_source.clone(),
        }
    }
}

pub use Either::{Edge, Node};

pub type EitherRef<'a, N, E> = Either<&'a N, &'a E>;
pub type EitherMut<'a, N, E> = Either<&'a mut N, &'a mut E>;

pub type EitherPin<P, Q> = Either<Pin<P>, Pin<Q>>;
pub type EitherPinRef<'a, N, E> = EitherPin<&'a N, &'a E>;
pub type EitherPinMut<'a, N, E> = EitherPin<&'a mut N, &'a mut E>;

impl<N, E> Either<N, E> {
    pub const fn is_node(&self) -> bool {
        matches!(self, Self::Node(_))
    }

    pub const fn is_edge(&self) -> bool {
        matches!(self, Self::Edge(_))
    }

    pub const fn node_ref(&self) -> Option<&N> {
        either!(self, node => Some(node), _ => None)
    }

    pub const fn node_mut(&mut self) -> Option<&mut N> {
        either!(self, node => Some(node), _ => None)
    }

    pub fn node(self) -> Option<N> {
        either!(self, node => Some(node), _ => None)
    }

    pub const fn edge_ref(&self) -> Option<&E> {
        either!(self, _ => None, edge => Some(edge))
    }

    pub const fn edge_mut(&mut self) -> Option<&mut E> {
        either!(self, _ => None, edge => Some(edge))
    }

    pub fn edge(self) -> Option<E> {
        either!(self, _ => None, edge => Some(edge))
    }
}

impl<N, E> Either<N, E> {
    pub const fn as_ref(&self) -> EitherRef<'_, N, E> {
        map_both!(self, inner => inner)
    }

    pub const fn as_mut(&mut self) -> EitherMut<'_, N, E> {
        map_both!(self, inner => inner)
    }

    pub const fn as_pin_ref(self: Pin<&Self>) -> EitherPinRef<'_, N, E> {
        // SAFETY: `inner` is guaranteed to be pinned because it comes from `self` which is pinned
        map_both!(self.get_ref(), inner => unsafe { Pin::new_unchecked(inner) })
    }

    pub const fn as_pin_mut(self: Pin<&mut Self>) -> EitherPinMut<'_, N, E> {
        map_both!(
            // SAFETY: `get_unchecked_mut` is never used to move `Self` from the obtained mutable reference
            unsafe { self.get_unchecked_mut() },
            // SAFETY: `inner` is guaranteed to be pinned because it comes from `self` which is pinned
            inner => unsafe { Pin::new_unchecked(inner) }
        )
    }

    pub fn map_node<M, F: FnOnce(N) -> M>(self, function: F) -> Either<M, E> {
        map_either!(self, node => function(node), edge => edge)
    }

    pub fn map_edge<D, F: FnOnce(E) -> D>(self, function: F) -> Either<N, D> {
        map_either!(self, node => node, edge => function(edge))
    }

    pub fn map<M, D, F: FnOnce(N) -> M, G: FnOnce(E) -> D>(
        self,
        node_function: F,
        edge_function: G,
    ) -> Either<M, D> {
        map_either!(self, node => node_function(node), edge => edge_function(edge))
    }

    pub fn map_into<T>(self) -> Either<T, T>
    where
        N: Into<T>,
        E: Into<T>,
    {
        map_both!(self, inner => inner.into())
    }

    pub fn map_into_inner<T>(self) -> T
    where
        N: Into<T>,
        E: Into<T>,
    {
        self.map_into().into_inner()
    }
}

impl<N: Deref, E: Deref> Either<N, E> {
    pub fn as_deref(&self) -> EitherRef<'_, N::Target, E::Target> {
        map_both!(self, inner => inner.deref())
    }
}

impl<N: DerefMut, E: DerefMut> Either<N, E> {
    pub fn as_deref_mut(&mut self) -> EitherMut<'_, N::Target, E::Target> {
        map_both!(self, inner => inner.deref_mut())
    }
}

impl<N, E> EitherRef<'_, N, E> {
    pub const fn copied(self) -> Either<N, E>
    where
        N: Copy,
        E: Copy,
    {
        map_both!(self, inner => copy(inner))
    }

    pub fn cloned(self) -> Either<N, E>
    where
        N: Clone,
        E: Clone,
    {
        map_both!(self, inner => inner.clone())
    }
}

impl<N, E> EitherMut<'_, N, E> {
    pub const fn copied(self) -> Either<N, E>
    where
        N: Copy,
        E: Copy,
    {
        map_both!(self, inner => copy(inner))
    }

    pub fn cloned(self) -> Either<N, E>
    where
        N: Clone,
        E: Clone,
    {
        map_both!(self, inner => inner.clone())
    }
}

pub const UNWRAP_NODE: &str = "called `unwrap_node` on `Edge` value";
pub const UNWRAP_EDGE: &str = "called `unwrap_edge` on `Node` value";

impl<N, E> Either<N, E> {
    pub fn expect_node(self, message: &str) -> N
    where
        E: Debug,
    {
        either!(self, node => node, edge => fail(message, &edge))
    }

    pub fn unwrap_node(self) -> N
    where
        E: Debug,
    {
        self.expect_node(UNWRAP_NODE)
    }

    pub fn expect_edge(self, message: &str) -> E
    where
        N: Debug,
    {
        either!(self, node => fail(message, &node), edge => edge)
    }

    pub fn unwrap_edge(self) -> E
    where
        N: Debug,
    {
        self.expect_edge(UNWRAP_EDGE)
    }
}

impl<T> Either<T, T> {
    pub fn into_inner(self) -> T {
        both!(self, inner => inner)
    }

    pub fn map_both<U, F: FnOnce(T) -> U>(self, function: F) -> Either<U, U> {
        map_both!(self, inner => function(inner))
    }
}

impl<N, E> Either<N, E> {
    pub fn node_and_then<M, F: FnOnce(N) -> Either<M, E>>(self, function: F) -> Either<M, E> {
        node_and_then!(self, node => function(node))
    }

    pub fn node_or(self, other: N) -> N {
        either!(self, node => node, _ => other)
    }

    pub fn node_or_else<F: FnOnce(E) -> N>(self, function: F) -> N {
        either!(self, node => node, edge => function(edge))
    }

    pub fn node_or_default(self) -> N
    where
        N: Default,
    {
        either!(self, node => node, _ => N::default())
    }

    pub fn edge_or(self, other: E) -> E {
        either!(self, _ => other, edge => edge)
    }

    pub fn edge_or_else<F: FnOnce(N) -> E>(self, function: F) -> E {
        either!(self, node => function(node), edge => edge)
    }

    pub fn edge_or_default(self) -> E
    where
        E: Default,
    {
        either!(self, _ => E::default(), edge => edge)
    }

    pub fn edge_and_then<D, F: FnOnce(E) -> Either<N, D>>(self, function: F) -> Either<N, D> {
        edge_and_then!(self, edge => function(edge))
    }
}

impl<N, E> Either<Either<N, E>, E> {
    pub fn flatten_node(self) -> Either<N, E> {
        node_and_then!(self, item => item)
    }
}

impl<N, E> Either<N, Either<N, E>> {
    pub fn flatten_edge(self) -> Either<N, E> {
        edge_and_then!(self, item => item)
    }
}

impl<N: Future, E: Future<Output = N::Output>> Future for Either<N, E> {
    type Output = N::Output;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        both!(self.as_pin_mut(), inner => inner.poll(context))
    }
}

impl<T, N, E> AsRef<T> for Either<N, E>
where
    N: AsRef<T>,
    E: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        both!(self, inner => inner.as_ref())
    }
}

impl<T, N, E> AsMut<T> for Either<N, E>
where
    N: AsMut<T>,
    E: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        both!(self, inner => inner.as_mut())
    }
}

impl<T, N, E> AsRef<[T]> for Either<N, E>
where
    N: AsRef<[T]>,
    E: AsRef<[T]>,
{
    fn as_ref(&self) -> &[T] {
        both!(self, inner => inner.as_ref())
    }
}

impl<T, N, E> AsMut<[T]> for Either<N, E>
where
    N: AsMut<[T]>,
    E: AsMut<[T]>,
{
    fn as_mut(&mut self) -> &mut [T] {
        both!(self, inner => inner.as_mut())
    }
}

#[doc(hidden)]
pub mod import {
    pub use core::convert::{AsMut, AsRef};
}

macro_rules! impl_as_ref_and_mut_unsized {
    ($type: ty) => {
        impl<N, E> $crate::either::import::AsRef<$type> for $crate::either::Either<N, E>
        where
            N: $crate::either::import::AsRef<$type>,
            E: $crate::either::import::AsRef<$type>,
        {
            fn as_ref(&self) -> &$type {
                $crate::both!(self, inner => inner.as_ref())
            }
        }

        impl<N, E> $crate::either::import::AsMut<$type> for $crate::either::Either<N, E>
        where
            N: $crate::either::import::AsMut<$type>,
            E: $crate::either::import::AsMut<$type>,
        {
            fn as_mut(&mut self) -> &mut $type {
                $crate::both!(self, inner => inner.as_mut())
            }
        }
    };
}

impl_as_ref_and_mut_unsized!(str);

impl_as_ref_and_mut_unsized!(CStr);

#[cfg(feature = "std")]
mod impl_as_ref_and_mut_unsized_std {
    use std::{ffi::OsStr, path::Path};

    impl_as_ref_and_mut_unsized!(OsStr);
    impl_as_ref_and_mut_unsized!(Path);
}

use core::{
    ffi::CStr,
    fmt::Debug,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
};

use crate::{base::Base, data::Data};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Item<N, E> {
    Node(N),
    Edge(E),
}

pub struct IterItem<N, E> {
    inner: Item<N, E>,
}

impl<N, E> IterItem<N, E> {
    pub(crate) const fn new(inner: Item<N, E>) -> Self {
        Self { inner }
    }
}

pub use Item::{Edge, Node};

pub type ItemRef<'a, N, E> = Item<&'a N, &'a E>;
pub type ItemMut<'a, N, E> = Item<&'a mut N, &'a mut E>;

pub type ItemPin<P, Q> = Item<Pin<P>, Pin<Q>>;
pub type ItemPinRef<'a, N, E> = ItemPin<&'a N, &'a E>;
pub type ItemPinMut<'a, N, E> = ItemPin<&'a mut N, &'a mut E>;

pub type IdOf<G> = Item<<G as Base>::NodeId, <G as Base>::EdgeId>;
pub type ValueOf<G> = Item<<G as Data>::NodeValue, <G as Data>::EdgeValue>;
pub type ValueRefOf<'g, G> = ItemRef<'g, <G as Data>::NodeValue, <G as Data>::EdgeValue>;
pub type ValueMutOf<'g, G> = ItemMut<'g, <G as Data>::NodeValue, <G as Data>::EdgeValue>;

#[macro_export]
macro_rules! match_item {
    (
        $item: expr,
        $node_pattern: pat => $node_output: expr,
        $edge_pattern: pat => $edge_output: expr $(,)?
    ) => {
        match $item {
            $crate::items::Item::Node($node_pattern) => $node_output,
            $crate::items::Item::Edge($edge_pattern) => $edge_output,
        }
    };
}

#[macro_export]
macro_rules! map_item {
    (
        $item: expr,
        $node_pattern: pat => $node_output: expr,
        $edge_pattern: pat => $edge_output: expr $(,)?
    ) => {
        $crate::match_item!(
            $item,
            $node_pattern => $crate::items::Item::Node($node_output),
            $edge_pattern => $crate::items::Item::Edge($edge_output),
        )
    };
}

#[macro_export]
macro_rules! match_both {
    ($item: expr, $both_pattern: pat => $both_output: expr) => {
        $crate::match_item!(
            $item,
            $both_pattern => $both_output,
            $both_pattern => $both_output,
        )
    };
}

#[macro_export]
macro_rules! map_both {
    ($item: expr, $both_pattern: pat => $both_output: expr) => {
        $crate::map_item!(
            $item,
            $both_pattern => $both_output,
            $both_pattern => $both_output,
        )
    }
}

macro_rules! node_and_then {
    ($item: expr, $node_pattern: pat => $node_output: expr) => {
        $crate::match_item!(
            $item,
            $node_pattern => $node_output,
            value => $crate::items::Item::Edge(value),
        )
    }
}

macro_rules! edge_and_then {
    ($item: expr, $edge_pattern: pat => $edge_output: expr) => {
        $crate::match_item!(
            $item,
            value => $crate::items::Item::Node(value),
            $edge_pattern => $edge_output,
        )
    }
}

impl<N, E> Item<N, E> {
    pub const fn is_node(&self) -> bool {
        matches!(self, Self::Node(_))
    }

    pub const fn is_edge(&self) -> bool {
        matches!(self, Self::Edge(_))
    }

    pub const fn node_ref(&self) -> Option<&N> {
        match_item!(self, node => Some(node), _ => None)
    }

    pub const fn node_mut(&mut self) -> Option<&mut N> {
        match_item!(self, node => Some(node), _ => None)
    }

    pub fn node(self) -> Option<N> {
        match_item!(self, node => Some(node), _ => None)
    }

    pub const fn edge_ref(&self) -> Option<&E> {
        match_item!(self, _ => None, edge => Some(edge))
    }

    pub const fn edge_mut(&mut self) -> Option<&mut E> {
        match_item!(self, _ => None, edge => Some(edge))
    }

    pub fn edge(self) -> Option<E> {
        match_item!(self, _ => None, edge => Some(edge))
    }
}

impl<N, E> Item<N, E> {
    pub const fn as_ref(&self) -> ItemRef<'_, N, E> {
        map_both!(self, inner => inner)
    }

    pub const fn as_mut(&mut self) -> ItemMut<'_, N, E> {
        map_both!(self, inner => inner)
    }

    pub const fn as_pin_ref(self: Pin<&Self>) -> ItemPinRef<'_, N, E> {
        // SAFETY: `inner` is guaranteed to be pinned becomes it comes from `self` which is pinned
        map_both!(self.get_ref(), inner => unsafe { Pin::new_unchecked(inner) })
    }

    pub const fn as_pin_mut(self: Pin<&mut Self>) -> ItemPinMut<'_, N, E> {
        map_both!(
            // SAFETY: `get_unchecked_mut` is never used to move the `Self` inside `self`
            unsafe { self.get_unchecked_mut() },
            // SAFETY: `inner` is guaranteed to be pinned becomes it comes from `self` which is pinned
            inner => unsafe { Pin::new_unchecked(inner) }
        )
    }

    pub fn map_node<M, F: FnOnce(N) -> M>(self, function: F) -> Item<M, E> {
        map_item!(self, node => function(node), edge => edge)
    }

    pub fn map_edge<D, F: FnOnce(E) -> D>(self, function: F) -> Item<N, D> {
        map_item!(self, node => node, edge => function(edge))
    }

    pub fn map<M, D, F: FnOnce(N) -> M, G: FnOnce(E) -> D>(
        self,
        node_function: F,
        edge_function: G,
    ) -> Item<M, D> {
        map_item!(self, node => node_function(node), edge => edge_function(edge))
    }

    pub fn map_into<T>(self) -> Item<T, T>
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

impl<N: Deref, E: Deref> Item<N, E> {
    pub fn as_deref(&self) -> Item<&N::Target, &E::Target> {
        map_both!(self, inner => inner.deref())
    }
}

impl<N: DerefMut, E: DerefMut> Item<N, E> {
    pub fn as_deref_mut(&mut self) -> Item<&mut N::Target, &mut E::Target> {
        map_both!(self, inner => inner.deref_mut())
    }
}

impl<N, E> ItemRef<'_, N, E> {
    pub const fn copied(self) -> Item<N, E>
    where
        N: Copy,
        E: Copy,
    {
        map_item!(self, node => *node, edge => *edge)
    }

    pub fn cloned(self) -> Item<N, E>
    where
        N: Clone,
        E: Clone,
    {
        map_item!(self, node => node.clone(), edge => edge.clone())
    }
}

impl<N, E> ItemMut<'_, N, E> {
    pub const fn copied(self) -> Item<N, E>
    where
        N: Copy,
        E: Copy,
    {
        map_item!(self, node => *node, edge => *edge)
    }

    pub fn cloned(self) -> Item<N, E>
    where
        N: Clone,
        E: Clone,
    {
        map_item!(self, node => node.clone(), edge => edge.clone())
    }
}

pub const UNWRAP_NODE: &str = "called `unwrap_node` on `Edge` value";
pub const UNWRAP_EDGE: &str = "called `unwrap_edge` on `Node` value";

impl<N, E> Item<N, E> {
    pub fn expect_node(self, message: &str) -> N
    where
        E: Debug,
    {
        match_item!(self, node => node, edge => panic!("{message}: {edge:?}"))
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
        match_item!(self, node => panic!("{message}: {node:?}"), edge => edge)
    }

    pub fn unwrap_edge(self) -> E
    where
        N: Debug,
    {
        self.expect_edge(UNWRAP_EDGE)
    }
}

impl<T> Item<T, T> {
    pub fn into_inner(self) -> T {
        match_both!(self, inner => inner)
    }

    pub fn map_both<U, F: FnOnce(T) -> U>(self, function: F) -> Item<U, U> {
        map_both!(self, inner => function(inner))
    }
}

impl<N, E> Item<N, E> {
    pub fn node_and_then<M, F: FnOnce(N) -> Item<M, E>>(self, function: F) -> Item<M, E> {
        node_and_then!(self, node => function(node))
    }

    pub fn node_or(self, other: N) -> N {
        match_item!(self, node => node, _ => other)
    }

    pub fn node_or_else<F: FnOnce(E) -> N>(self, function: F) -> N {
        match_item!(self, node => node, edge => function(edge))
    }

    pub fn node_or_default(self) -> N
    where
        N: Default,
    {
        match_item!(self, node => node, _ => N::default())
    }

    pub fn edge_or(self, other: E) -> E {
        match_item!(self, _ => other, node => node)
    }

    pub fn edge_or_else<F: FnOnce(N) -> E>(self, function: F) -> E {
        match_item!(self, node => function(node), edge => edge)
    }

    pub fn edge_or_default(self) -> E
    where
        E: Default,
    {
        match_item!(self, _ => E::default(), edge => edge)
    }

    pub fn edge_and_then<D, F: FnOnce(E) -> Item<N, D>>(self, function: F) -> Item<N, D> {
        edge_and_then!(self, edge => function(edge))
    }
}

impl<N, E> Item<N, E> {
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> Item<N::IntoIter, E::IntoIter>
    where
        N: IntoIterator,
        E: IntoIterator<Item = N::Item>,
    {
        map_both!(self, inner => inner.into_iter())
    }

    pub fn iter(&self) -> Item<<&N as IntoIterator>::IntoIter, <&E as IntoIterator>::IntoIter>
    where
        for<'a> &'a N: IntoIterator,
        for<'a> &'a E: IntoIterator<Item = <&'a N as IntoIterator>::Item>,
    {
        map_both!(self, inner => inner.into_iter())
    }

    pub fn iter_mut(
        &mut self,
    ) -> Item<<&mut N as IntoIterator>::IntoIter, <&mut E as IntoIterator>::IntoIter>
    where
        for<'a> &'a mut N: IntoIterator,
        for<'a> &'a mut E: IntoIterator<Item = <&'a mut N as IntoIterator>::Item>,
    {
        map_both!(self, inner => inner.into_iter())
    }
}

impl<N, E> Item<Option<N>, Option<E>> {
    pub fn factor_none(self) -> Option<Item<N, E>> {
        match_item!(
            self,
            node_option => node_option.map(Node),
            edge_option => edge_option.map(Edge),
        )
    }
}

impl<T, N, E> Item<Result<T, N>, Result<T, E>> {
    pub fn factor_ok(self) -> Result<T, Item<N, E>> {
        match_item!(
            self,
            node_result => node_result.map_err(Node),
            edge_result => edge_result.map_err(Edge),
        )
    }
}

impl<T, N, E> Item<Result<N, T>, Result<E, T>> {
    pub fn factor_err(self) -> Result<Item<N, E>, T> {
        match_item!(
            self,
            node_result => node_result.map(Node),
            edge_result => edge_result.map(Edge),
        )
    }
}

impl<T, N, E> Item<(T, N), (T, E)> {
    pub fn factor_first(self) -> (T, Item<N, E>) {
        match_item!(
            self,
            (value, node) => (value, Node(node)),
            (value, edge) => (value, Edge(edge)),
        )
    }
}

impl<T, N, E> Item<(N, T), (E, T)> {
    pub fn factor_second(self) -> (Item<N, E>, T) {
        match_item!(
            self,
            (node, value) => (Node(node), value),
            (edge, value) => (Edge(edge), value),
        )
    }
}

impl<N, E> Item<Item<N, E>, E> {
    pub fn flatten_node(self) -> Item<N, E> {
        node_and_then!(self, item => item)
    }
}

impl<N, E> Item<N, Item<N, E>> {
    pub fn flatten_edge(self) -> Item<N, E> {
        edge_and_then!(self, item => item)
    }
}

impl<N: Iterator, E: Iterator<Item = N::Item>> Iterator for Item<N, E> {
    type Item = N::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match_both!(self, inner => inner.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match_both!(self, inner => inner.size_hint())
    }
}

impl<T, N: Extend<T>, E: Extend<T>> Extend<T> for Item<N, E> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
        match_both!(self, inner => inner.extend(iterable));
    }
}

impl<N: Future, E: Future<Output = N::Output>> Future for Item<N, E> {
    type Output = N::Output;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match_both!(self.as_pin_mut(), inner => inner.poll(context))
    }
}

impl<T, N, E> AsRef<T> for Item<N, E>
where
    N: AsRef<T>,
    E: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        match_both!(self, inner => inner.as_ref())
    }
}

impl<T, N, E> AsMut<T> for Item<N, E>
where
    N: AsMut<T>,
    E: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        match_both!(self, inner => inner.as_mut())
    }
}

impl<T, N, E> AsRef<[T]> for Item<N, E>
where
    N: AsRef<[T]>,
    E: AsRef<[T]>,
{
    fn as_ref(&self) -> &[T] {
        match_both!(self, inner => inner.as_ref())
    }
}

impl<T, N, E> AsMut<[T]> for Item<N, E>
where
    N: AsMut<[T]>,
    E: AsMut<[T]>,
{
    fn as_mut(&mut self) -> &mut [T] {
        match_both!(self, inner => inner.as_mut())
    }
}

#[doc(hidden)]
pub mod import {
    pub use core::convert::{AsMut, AsRef};
}

macro_rules! impl_as_ref_and_mut_unsized {
    ($type: ty) => {
        impl<N, E> $crate::items::import::AsRef<$type> for $crate::items::Item<N, E>
        where
            N: $crate::items::import::AsRef<$type>,
            E: $crate::items::import::AsRef<$type>,
        {
            fn as_ref(&self) -> &$type {
                match_both!(self, inner => inner.as_ref())
            }
        }

        impl<N, E> $crate::items::import::AsMut<$type> for $crate::items::Item<N, E>
        where
            N: $crate::items::import::AsMut<$type>,
            E: $crate::items::import::AsMut<$type>,
        {
            fn as_mut(&mut self) -> &mut $type {
                match_both!(self, inner => inner.as_mut())
            }
        }
    };
}

impl_as_ref_and_mut_unsized!(str);

impl_as_ref_and_mut_unsized!(CStr);

#[cfg(feature = "std")]
mod impl_std {
    use std::{ffi::OsStr, path::Path};

    impl_as_ref_and_mut_unsized!(OsStr);
    impl_as_ref_and_mut_unsized!(Path);
}

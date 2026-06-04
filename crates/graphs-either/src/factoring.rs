use core::iter::FusedIterator;

use crate::{
    both, either,
    either::{Edge, Either, Node},
    factor_iter, map_both,
};

#[derive(Debug, Clone)]
pub struct FactorIterator<N: Iterator, E: Iterator> {
    either: Either<N, E>,
}

pub type FactorIntoIter<N, E> =
    FactorIterator<<N as IntoIterator>::IntoIter, <E as IntoIterator>::IntoIter>;

pub type FactorIter<'a, N, E> = FactorIntoIter<&'a N, &'a E>;
pub type FactorIterMut<'a, N, E> = FactorIntoIter<&'a mut N, &'a mut E>;

impl<N: Iterator, E: Iterator> FactorIterator<N, E> {
    pub(crate) const fn new(either: Either<N, E>) -> Self {
        Self { either }
    }
}

impl<N: Iterator, E: Iterator> Iterator for FactorIterator<N, E> {
    type Item = Either<N::Item, E::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        map_both!(self.either, ref mut inner => inner.next()).factor_none()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        both!(self.either, ref inner => inner.size_hint())
    }
}

impl<N: DoubleEndedIterator, E: DoubleEndedIterator> DoubleEndedIterator for FactorIterator<N, E> {
    fn next_back(&mut self) -> Option<Self::Item> {
        map_both!(self.either, ref mut inner => inner.next_back()).factor_none()
    }
}

impl<N: ExactSizeIterator, E: ExactSizeIterator> ExactSizeIterator for FactorIterator<N, E> {
    fn len(&self) -> usize {
        both!(self.either, ref inner => inner.len())
    }
}

impl<N: FusedIterator, E: FusedIterator> FusedIterator for FactorIterator<N, E> {}

impl<N, E> Either<N, E> {
    pub fn factor_into_iter(self) -> FactorIntoIter<N, E>
    where
        N: IntoIterator,
        E: IntoIterator,
    {
        factor_iter!(self)
    }

    // NOTE: even more HRTBs! <3 ~ nekit

    pub fn factor_iter(&self) -> FactorIter<'_, N, E>
    where
        for<'a> &'a N: IntoIterator,
        for<'a> &'a E: IntoIterator,
    {
        factor_iter!(self)
    }

    pub fn factor_iter_mut(&mut self) -> FactorIterMut<'_, N, E>
    where
        for<'a> &'a mut N: IntoIterator,
        for<'a> &'a mut E: IntoIterator,
    {
        factor_iter!(self)
    }
}

impl<N, E> Either<Option<N>, Option<E>> {
    pub fn factor_none(self) -> Option<Either<N, E>> {
        either!(
            self,
            node_option => node_option.map(Node),
            edge_option => edge_option.map(Edge),
        )
    }
}

impl<T, N, E> Either<Result<T, N>, Result<T, E>> {
    pub fn factor_ok(self) -> Result<T, Either<N, E>> {
        either!(
            self,
            node_result => node_result.map_err(Node),
            edge_result => edge_result.map_err(Edge),
        )
    }
}

impl<T, N, E> Either<Result<N, T>, Result<E, T>> {
    pub fn factor_err(self) -> Result<Either<N, E>, T> {
        either!(
            self,
            node_result => node_result.map(Node),
            edge_result => edge_result.map(Edge),
        )
    }
}

impl<T, N, E> Either<(T, N), (T, E)> {
    pub fn factor_first(self) -> (T, Either<N, E>) {
        either!(
            self,
            (value, node) => (value, Node(node)),
            (value, edge) => (value, Edge(edge)),
        )
    }
}

impl<T, N, E> Either<(N, T), (E, T)> {
    pub fn factor_second(self) -> (Either<N, E>, T) {
        either!(
            self,
            (node, value) => (Node(node), value),
            (edge, value) => (Edge(edge), value),
        )
    }
}

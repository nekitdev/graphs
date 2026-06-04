use core::iter::FusedIterator;

use crate::{both, either::Either, map_both};

pub type EitherIntoIter<N, E> =
    Either<<N as IntoIterator>::IntoIter, <E as IntoIterator>::IntoIter>;

pub type EitherIter<'a, N, E> = EitherIntoIter<&'a N, &'a E>;
pub type EitherIterMut<'a, N, E> = EitherIntoIter<&'a mut N, &'a mut E>;

// XXX: do not implement `IntoIterator` as that causes conflicts with the `Iterator` implementation
#[allow(clippy::should_implement_trait)]
impl<N, E> Either<N, E> {
    pub fn into_iter(self) -> EitherIntoIter<N, E>
    where
        N: IntoIterator,
        E: IntoIterator<Item = N::Item>,
    {
        map_both!(self, inner => inner.into_iter())
    }

    // NOTE: I love higher-ranked trait bounds! <3 ~ nekit

    pub fn iter(&self) -> EitherIter<'_, N, E>
    where
        for<'a> &'a N: IntoIterator,
        for<'a> &'a E: IntoIterator<Item = <&'a N as IntoIterator>::Item>,
    {
        map_both!(self, inner => inner.into_iter())
    }

    pub fn iter_mut(&mut self) -> EitherIterMut<'_, N, E>
    where
        for<'a> &'a mut N: IntoIterator,
        for<'a> &'a mut E: IntoIterator<Item = <&'a mut N as IntoIterator>::Item>,
    {
        map_both!(self, inner => inner.into_iter())
    }
}

impl<N: Iterator, E: Iterator<Item = N::Item>> Iterator for Either<N, E> {
    type Item = N::Item;

    fn next(&mut self) -> Option<Self::Item> {
        both!(self, inner => inner.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        both!(self, inner => inner.size_hint())
    }
}

impl<N: DoubleEndedIterator, E: DoubleEndedIterator<Item = N::Item>> DoubleEndedIterator
    for Either<N, E>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        both!(self, inner => inner.next())
    }
}

impl<N: ExactSizeIterator, E: ExactSizeIterator<Item = N::Item>> ExactSizeIterator
    for Either<N, E>
{
    fn len(&self) -> usize {
        both!(self, inner => inner.len())
    }
}

impl<N: FusedIterator, E: FusedIterator<Item = N::Item>> FusedIterator for Either<N, E> {}

impl<T, N: Extend<T>, E: Extend<T>> Extend<T> for Either<N, E> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
        both!(self, inner => inner.extend(iterable));
    }
}

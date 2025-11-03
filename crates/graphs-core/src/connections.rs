use core::{fmt, marker::PhantomData, mem::swap};

use crate::kinds::{self, DefaultKind, Kind};

pub trait Connection: Sized {
    type Item;
    type Kind: Kind;

    type Inverse: Connection<Item = Self::Item, Kind = <Self::Kind as Kind>::Inverse>;

    fn connecting(one: Self::Item, two: Self::Item) -> Self;

    fn parts(&self) -> (&Self::Item, &Self::Item);

    fn parts_mut(&mut self) -> (&mut Self::Item, &mut Self::Item);

    fn into_parts(self) -> (Self::Item, Self::Item);

    fn reverse(&mut self) {
        let (one, two) = self.parts_mut();

        swap(one, two);
    }

    fn invert(self) -> Self::Inverse {
        let (one, two) = self.into_parts();

        Self::Inverse::connecting(two, one)
    }
}

pub trait Directed: Connection<Kind = kinds::Directed> {}
pub trait Undirected: Connection<Kind = kinds::Undirected> {}

impl<C: Connection<Kind = kinds::Directed>> Directed for C {}
impl<C: Connection<Kind = kinds::Undirected>> Undirected for C {}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
// pub struct DefaultDirected<T> {
//     pub source: T,
//     pub target: T,
// }

// impl<T: fmt::Display> fmt::Display for DefaultDirected<T> {
//     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             formatter,
//             "{source} -> {target}",
//             source = self.source,
//             target = self.target
//         )
//     }
// }

// impl<T> DefaultDirected<T> {
//     pub const fn new(source: T, target: T) -> Self {
//         Self { source, target }
//     }

//     pub const fn reverse(&mut self) {
//         swap(&mut self.source, &mut self.target);
//     }
// }

// impl<T> Connection for DefaultDirected<T> {
//     type Item = T;
//     type Kind = kinds::Directed;

//     type Inverse = DefaultUndirected<T>;

//     fn connecting(one: Self::Item, two: Self::Item) -> Self {
//         Self::new(one, two)
//     }

//     fn parts(&self) -> (&Self::Item, &Self::Item) {
//         (&self.source, &self.target)
//     }

//     fn parts_mut(&mut self) -> (&mut Self::Item, &mut Self::Item) {
//         (&mut self.source, &mut self.target)
//     }

//     fn into_parts(self) -> (Self::Item, Self::Item) {
//         (self.source, self.target)
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
// pub struct DefaultUndirected<T> {
//     pub one: T,
//     pub two: T,
// }

// impl<T: fmt::Display> fmt::Display for DefaultUndirected<T> {
//     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(formatter, "{one} <-> {two}", one = self.one, two = self.two)
//     }
// }

// impl<T> DefaultUndirected<T> {
//     pub const fn new(one: T, two: T) -> Self {
//         Self { one, two }
//     }
// }

// impl<T> Connection for DefaultUndirected<T> {
//     type Item = T;
//     type Kind = kinds::Undirected;

//     type Inverse = DefaultDirected<T>;

//     fn connecting(one: Self::Item, two: Self::Item) -> Self {
//         Self::new(one, two)
//     }

//     fn parts(&self) -> (&Self::Item, &Self::Item) {
//         (&self.one, &self.two)
//     }

//     fn parts_mut(&mut self) -> (&mut Self::Item, &mut Self::Item) {
//         (&mut self.one, &mut self.two)
//     }

//     fn into_parts(self) -> (Self::Item, Self::Item) {
//         (self.one, self.two)
//     }
// }

pub struct Kinded<T, K: Kind = DefaultKind> {
    pub one: T,
    pub two: T,
    kind: PhantomData<K>,
}

impl<T: fmt::Display, K: Kind> fmt::Display for Kinded<T, K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (one, two) = self.parts();

        if K::DIRECTED {
            write!(formatter, "{one} -> {two}")
        } else {
            write!(formatter, "{one} <-> {two}")
        }
    }
}

impl<T, K: Kind> Kinded<T, K> {
    pub const fn new(one: T, two: T) -> Self {
        Self {
            one,
            two,
            kind: PhantomData,
        }
    }

    pub const fn reverse(&mut self) {
        swap(&mut self.one, &mut self.two);
    }
}

impl<T, K: Kind> Connection for Kinded<T, K> {
    type Item = T;
    type Kind = K;

    type Inverse = Kinded<T, <K as Kind>::Inverse>;

    fn connecting(one: Self::Item, two: Self::Item) -> Self {
        Self::new(one, two)
    }

    fn parts(&self) -> (&Self::Item, &Self::Item) {
        (&self.one, &self.two)
    }

    fn parts_mut(&mut self) -> (&mut Self::Item, &mut Self::Item) {
        (&mut self.one, &mut self.two)
    }

    fn into_parts(self) -> (Self::Item, Self::Item) {
        (self.one, self.two)
    }
}

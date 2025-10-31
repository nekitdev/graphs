use core::marker::PhantomData;

use crate::{
    kinds::{DefaultKind, Kind},
    loops::{DefaultLoop, Loop},
    types::{DefaultType, Type},
};

/// Combines [`Kind`], [`Type`] and [`Loop`] markers into one specification structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Specs<K: Kind = DefaultKind, T: Type = DefaultType, L: Loop = DefaultLoop> {
    phantom_kind: PhantomData<K>,
    phantom_type: PhantomData<T>,
    phantom_loop: PhantomData<L>,
}

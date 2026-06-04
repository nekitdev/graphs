use core::{fmt, marker::PhantomData};

use graphs_common::index::{EdgeIndex, NodeIndex};
use graphs_core::{
    find::Missing,
    index::{DefaultUntypedIndex, Index, UntypedIndex},
    kinds::{DefaultKind, Kind},
    sentinel::Sentinel,
};
use thiserror::Error;

use crate::{
    internals::{debug_struct, debug_wrap_enum},
    parts::Connection,
};

#[derive(Error)]
pub struct LoopError<I: UntypedIndex = DefaultUntypedIndex, K: Kind = DefaultKind> {
    pub connection: Connection<I, K>,
}

impl<I: UntypedIndex, K: Kind> LoopError<I, K> {
    pub const fn new(connection: Connection<I, K>) -> Self {
        Self { connection }
    }
}

impl<I: UntypedIndex, K: Kind> fmt::Debug for LoopError<I, K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_struct!(formatter => LoopError {
            connection: self.connection,
        })
        .finish()
    }
}

impl<I: UntypedIndex, K: Kind> fmt::Display for LoopError<I, K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "loop `{connection}` is not allowed",
            connection = self.connection
        )
    }
}

pub type MissingError<I = DefaultUntypedIndex, K = DefaultKind> = Missing<Connection<I, K>>;

#[derive(Error)]
#[error("edge `{index}` connecting `{connection}` already exists")]
pub struct MultipleError<I: UntypedIndex = DefaultUntypedIndex, K: Kind = DefaultKind> {
    pub index: EdgeIndex<I>,
    pub connection: Connection<I, K>,
}

impl<I: UntypedIndex, K: Kind> MultipleError<I, K> {
    pub const fn new(index: EdgeIndex<I>, connection: Connection<I, K>) -> Self {
        Self { index, connection }
    }
}

impl<I: UntypedIndex, K: Kind> fmt::Debug for MultipleError<I, K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_struct!(formatter => MultipleError {
            index: self.index,
            connection: self.connection,
        })
        .finish()
    }
}

#[derive(Error)]
#[error(transparent)]
pub enum SimpleError<I: UntypedIndex = DefaultUntypedIndex, K: Kind = DefaultKind> {
    Loop(#[from] LoopError<I, K>),
    Multiple(#[from] MultipleError<I, K>),
    Missing(#[from] MissingError<I, K>),
    Index(#[from] IndexError<I>),
}

impl<I: UntypedIndex, K: Kind> fmt::Debug for SimpleError<I, K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_wrap_enum!(formatter => self => {
            Loop,
            Multiple,
            Missing,
            Index,
        })
    }
}

#[derive(Error)]
#[error(transparent)]
pub enum LoopedError<I: UntypedIndex = DefaultUntypedIndex, K: Kind = DefaultKind> {
    Multiple(#[from] MultipleError<I, K>),
    Missing(#[from] MissingError<I, K>),
    Index(#[from] IndexError<I>),
}

impl<I: UntypedIndex, K: Kind> fmt::Debug for LoopedError<I, K> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_wrap_enum!(formatter => self => {
            Multiple,
            Missing,
            Index,
        })
    }
}

#[derive(Debug, Error)]
#[error("invalid index value `{value}`")]
pub struct ValueError<I: UntypedIndex = DefaultUntypedIndex> {
    pub value: usize,
    index: PhantomData<I>,
}

impl<I: UntypedIndex> ValueError<I> {
    pub const fn new(value: usize) -> Self {
        Self {
            value,
            index: PhantomData,
        }
    }
}

#[derive(Debug, Error)]
#[error("sentinel ({sentinel}) reached", sentinel = I::SENTINEL)]
pub struct SentinelError<I: UntypedIndex = DefaultUntypedIndex> {
    index: PhantomData<I>,
}

impl<I: UntypedIndex> SentinelError<I> {
    pub const fn new() -> Self {
        Self { index: PhantomData }
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum IndexError<I: UntypedIndex = DefaultUntypedIndex> {
    Value(#[from] ValueError<I>),
    Sentinel(#[from] SentinelError<I>),
}

impl<I: UntypedIndex> IndexError<I> {
    pub const fn value(value: usize) -> Self {
        Self::Value(ValueError::new(value))
    }

    pub const fn sentinel() -> Self {
        Self::Sentinel(SentinelError::new())
    }
}

pub fn node_index<I: UntypedIndex>(value: usize) -> Result<NodeIndex<I>, IndexError<I>> {
    if let Some(index) = NodeIndex::try_of(value) {
        if index.is_sentinel() {
            Err(IndexError::sentinel())
        } else {
            Ok(index)
        }
    } else {
        Err(IndexError::value(value))
    }
}

pub fn edge_index<I: UntypedIndex>(value: usize) -> Result<EdgeIndex<I>, IndexError<I>> {
    if let Some(index) = EdgeIndex::try_of(value) {
        if index.is_sentinel() {
            Err(IndexError::sentinel())
        } else {
            Ok(index)
        }
    } else {
        Err(IndexError::value(value))
    }
}

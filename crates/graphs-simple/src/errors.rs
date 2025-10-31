use graphs_core::{
    find::Missing,
    keys::{DefaultUntypedIndex, NodeIndex, UntypedIndex},
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("node limit reached")]
pub struct NodeError;

impl NodeError {
    pub const fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("edge limit reached")]
pub struct LimitError;

impl LimitError {
    pub const fn new() -> Self {
        Self
    }
}

pub type MissingError<I = DefaultUntypedIndex> = Missing<NodeIndex<I>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error(transparent)]
pub enum PseudoError<I: UntypedIndex = DefaultUntypedIndex> {
    Limit(#[from] LimitError),
    Missing(#[from] MissingError<I>),
}

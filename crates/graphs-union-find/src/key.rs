use thiserror::Error;

use graphs_core::index::{DefaultUntypedIndex, UntypedIndex};

pub const OUT_OF_BOUNDS: &str = "index out of bounds";

#[derive(Debug, Error)]
#[error("{OUT_OF_BOUNDS} `{index}` (count `{count}`)")]
pub struct OutOfBoundsError {
    pub index: usize,
    pub count: usize,
}

impl OutOfBoundsError {
    pub const fn new(index: usize, count: usize) -> Self {
        Self { index, count }
    }

    pub const fn get(self) -> (usize, usize) {
        (self.index, self.count)
    }
}

pub const fn check_index(index: usize, count: usize) -> Result<(), OutOfBoundsError> {
    if index < count {
        Ok(())
    } else {
        Err(OutOfBoundsError::new(index, count))
    }
}

pub const INVALID_KEY: &str = "invalid key index";

#[derive(Debug, Error)]
#[error("{INVALID_KEY} `{index}`")]
pub struct KeyError {
    pub index: usize,
}

impl KeyError {
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    pub const fn get(self) -> usize {
        self.index
    }
}

pub trait Key: UntypedIndex {
    fn try_new(index: usize) -> Result<Self, KeyError> {
        Self::try_of(index).ok_or_else(|| KeyError::new(index))
    }

    fn new(index: usize) -> Self {
        Self::try_new(index).expect(INVALID_KEY)
    }

    fn try_unkey(self) -> Result<usize, IndexError<Self>> {
        self.try_index().ok_or_else(|| IndexError::new(self))
    }

    fn unkey(self) -> usize {
        self.try_unkey().expect(INVALID_INDEX)
    }
}

pub type DefaultKey = DefaultUntypedIndex;

impl<K: UntypedIndex> Key for K {}

pub const INVALID_INDEX: &str = "invalid index key";

#[derive(Debug, Error)]
#[error("{INVALID_INDEX} `{key}`")]
pub struct IndexError<K: Key> {
    pub key: K,
}

impl<K: Key> IndexError<K> {
    pub const fn new(key: K) -> Self {
        Self { key }
    }

    pub const fn get(self) -> K {
        self.key
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error<K: Key = DefaultKey> {
    Index(#[from] IndexError<K>),
    OutOfBounds(#[from] OutOfBoundsError),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum AnyError<K: Key = DefaultKey> {
    Key(#[from] KeyError),
    Error(#[from] Error<K>),
}

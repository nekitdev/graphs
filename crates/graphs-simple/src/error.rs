//! Graph-related errors.

use core::{error, fmt, result};

use thiserror::Error;

/// Represents kinds of errors that can occur when performing operations on graphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum ErrorKind {
    /// The node limit has been reached.
    #[error("node limit reached")]
    NodeLimit,

    /// The edge limit has been reached.
    #[error("edge limit reached")]
    EdgeLimit,

    /// The node is missing.
    #[error("node `{0}` is missing")]
    NodeMissing(usize),

    /// The index is out-of-bounds.
    #[error("index out-of-bounds")]
    OutOfBounds,
}

/// Represents errors that can occur when performing operations on graphs.
///
/// This type is generic over the value of type `T`, which is used to
/// return ownership back to the caller.
pub struct Error<T> {
    /// The kind of the error.
    pub kind: ErrorKind,
    /// The value to return back.
    pub value: T,
}

impl<T> Error<T> {
    /// Constructs [`Self`] with the given kind and value.
    pub const fn new(kind: ErrorKind, value: T) -> Self {
        Self { kind, value }
    }

    /// Constructs [`Self`] with the given value and the [`NodeLimit`] kind.
    ///
    /// [`NodeLimit`]: ErrorKind::NodeLimit
    pub const fn node_limit(value: T) -> Self {
        Self::new(ErrorKind::NodeLimit, value)
    }

    /// Constructs [`Self`] with the given value and the [`EdgeLimit`] kind.
    ///
    /// [`EdgeLimit`]: ErrorKind::EdgeLimit
    pub const fn edge_limit(value: T) -> Self {
        Self::new(ErrorKind::EdgeLimit, value)
    }

    /// Constructs [`Self`] with the given value and the [`NodeMissing`] kind.
    ///
    /// [`NodeMissing`]: ErrorKind::NodeMissing
    pub const fn node_missing(index: usize, value: T) -> Self {
        Self::new(ErrorKind::NodeMissing(index), value)
    }

    /// Constructs [`Self`] with the given value and the [`OutOfBounds`] kind.
    ///
    /// [`OutOfBounds`]: ErrorKind::OutOfBounds
    pub const fn out_of_bounds(value: T) -> Self {
        Self::new(ErrorKind::OutOfBounds, value)
    }
}

/// The `Error` literal.
pub const NAME: &str = "Error";

/// The `kind` literal.
pub const KIND: &str = "kind";

/// The `error` literal.
pub const ERROR: &str = "error";

impl<T> fmt::Debug for Error<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(NAME)
            .field(KIND, &self.kind)
            .finish()
    }
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{ERROR}: {kind}", kind = self.kind)
    }
}

impl<T> error::Error for Error<T> {}

/// Represents the result type for graph operations.
pub type Result<T, R> = result::Result<T, Error<R>>;

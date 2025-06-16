//! Incidences in graphs.

use core::{num::ParseIntError, str::FromStr};

use thiserror::Error;

/// The `-1` literal.
pub const LEAVE: i8 = -1;

/// The `0` literal.
pub const NEITHER: i8 = 0;

/// The `1` literal.
pub const ENTER: i8 = 1;

/// Represents errors that can occur when constructing [`Incidence`] values.
#[derive(Debug, Error)]
#[error("`{value}` does not map to incidence")]
pub struct Error {
    /// The value that does not map to any incidence.
    pub value: i8,
}

impl Error {
    /// Constructs [`Self`].
    pub const fn new(value: i8) -> Self {
        Self { value }
    }
}

/// Represents errors that can occur when parsing [`Incidence`] values from strings.
#[derive(Debug, Error)]
#[error("failed to parse incidence")]
pub enum ParseError {
    /// The value could not be parsed to integer.
    Parse(#[from] ParseIntError),
    /// The value could be parsed to integer, but the result does not map to any incidence.
    Value(#[from] Error),
}

/// Represents incidences in graphs.
///
/// Incidences describe the relationship of some edge with respect to some node:
///
/// - [`LEAVE`] means that the edge leaves the node,
/// - [`ENTER`] means that the edge enters the node,
/// - [`NEITHER`] means that neither of the above applies (default).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(i8)]
pub enum Incidence {
    /// The edge leaves the node.
    Leave = LEAVE,

    /// The edge neither leaves nor enters the node (default).
    #[default]
    Neither = NEITHER,

    /// The edge enters the node.
    Enter = ENTER,
}

pub use Incidence::{Enter, Leave, Neither};

impl Incidence {
    /// Constructs [`Self`] from the given value, if possible.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if the value does not map to any [`Self`] value.
    pub const fn new(value: i8) -> Result<Self, Error> {
        match value {
            LEAVE => Ok(Self::Leave),
            NEITHER => Ok(Self::Neither),
            ENTER => Ok(Self::Enter),
            _ => Err(Error::new(value)),
        }
    }
}

impl FromStr for Incidence {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let value = string.parse()?;

        let incidence = Self::new(value)?;

        Ok(incidence)
    }
}

//! Empty values in graphs.

use core::fmt;

/// Represents empty values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Empty;

/// The `empty` string literal.
pub const EMPTY: &str = "empty";

impl fmt::Display for Empty {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(EMPTY)
    }
}

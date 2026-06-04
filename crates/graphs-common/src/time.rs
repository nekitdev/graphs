use core::fmt;

use thiserror::Error;

use crate::internals::copy;

/// Represents the underlying [`Time`] value.
pub type Value = usize;

pub const OVERFLOW: &str = "time overflow";

/// Represents errors returned when incrementing [`Time`] would cause overflows.
#[derive(Debug, Error)]
#[error("{OVERFLOW}")]
pub struct Overflow;

pub type Output<T> = Result<T, Overflow>;

/// Represents monotonically increasing time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    value: usize,
}

impl fmt::Display for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

pub const START: usize = 0;

impl Default for Time {
    fn default() -> Self {
        Self::start()
    }
}

impl Time {
    #[must_use]
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    #[must_use]
    pub const fn start() -> Self {
        Self::START
    }

    pub const fn post_increment(&mut self) -> Output<Self> {
        let resulting = copy(self);

        if let Err(overflow) = self.increment() {
            Err(overflow)
        } else {
            Ok(resulting)
        }
    }

    pub const fn increment(&mut self) -> Output<()> {
        let Some(incremented) = self.value.checked_add(1) else {
            return Err(Overflow);
        };

        self.value = incremented;

        Ok(())
    }

    #[must_use]
    pub const fn saturating_post_increment(&mut self) -> Self {
        let resulting = copy(self);

        self.saturating_increment();

        resulting
    }

    pub const fn saturating_increment(&mut self) {
        self.value = self.value.saturating_add(1);
    }

    #[must_use]
    pub const fn wrapping_post_increment(&mut self) -> Self {
        let resulting = copy(self);

        self.wrapping_increment();

        resulting
    }

    pub const fn wrapping_increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }

    #[must_use]
    #[track_caller]
    pub const fn strict_post_increment(&mut self) -> Self {
        let resulting = copy(self);

        self.strict_increment();

        resulting
    }

    #[track_caller]
    pub const fn strict_increment(&mut self) {
        self.value = self.value.strict_add(1);
    }

    #[must_use]
    pub const fn get(self) -> usize {
        self.value
    }

    pub const START: Self = Self::new(START);
}

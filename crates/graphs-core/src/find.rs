use core::fmt;

use thiserror::Error;

use crate::{base::Base, connections::Connection, references::EdgeReferences};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum Missing<C: Connection> {
    Entire(C),
    One(C::NodeId),
    Two(C::NodeId),
}

pub fn or_missing<T, C: Connection>(
    one_option: Option<T>,
    two_option: Option<T>,
    connection: C,
) -> Result<(T, T), Missing<C>> {
    let (one_id, two_id) = connection.parts();

    match (one_option, two_option) {
        (Some(one), Some(two)) => Ok((one, two)),
        (Some(_), None) => Err(Missing::Two(two_id)),
        (None, Some(_)) => Err(Missing::One(one_id)),
        (None, None) => Err(Missing::Entire(connection)),
    }
}

impl<C: Connection + fmt::Display> fmt::Display for Missing<C> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Entire(connection) => {
                write!(formatter, "entire `{connection}` is missing")
            }
            Self::One(one) => {
                write!(formatter, "one `{one}` is missing")
            }
            Self::Two(two) => {
                write!(formatter, "two `{two}` is missing")
            }
        }
    }
}

pub type Found<'g, G> = Result<<G as Find>::Connecting<'g>, Missing<<G as Base>::Connection>>;

pub trait Nothing: Sized {
    fn nothing() -> Self;
}

impl<T> Nothing for Option<T> {
    fn nothing() -> Self {
        Self::None
    }
}

mod sealed {
    pub trait Sealed {}
}

pub trait OrNothing: sealed::Sealed {
    type Value: Nothing;
    type Error;

    fn or_nothing(self) -> Self::Value;
}

impl<T, E> sealed::Sealed for Result<T, E> {}

impl<T: Nothing, E> OrNothing for Result<T, E> {
    type Value = T;
    type Error = E;

    fn or_nothing(self) -> Self::Value {
        self.unwrap_or_else(|_| Nothing::nothing())
    }
}

pub trait Find: EdgeReferences {
    type Connecting<'g>: Iterator<Item = Self::EdgeRef<'g>>
    where
        Self: 'g;

    fn find(&self, connection: Self::Connection) -> Found<'_, Self>;

    fn find_connecting(&self, one: Self::NodeId, two: Self::NodeId) -> Found<'_, Self> {
        self.find(Self::Connection::connecting(one, two))
    }
}

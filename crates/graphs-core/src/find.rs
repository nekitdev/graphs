use thiserror::Error;

use crate::{base::Base, connections::Connection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum Missing<C: Connection> {
    #[error("full `{0}` is missing")]
    Full(C),
    #[error("one `{0}` is missing")]
    One(C::Item),
    #[error("two `{0}` is missing")]
    Two(C::Item),
}

pub type FindResult<G> = Result<<G as Find>::Output, Missing<<G as Base>::Connection>>;

pub trait Output: Iterator {
    fn nothing() -> Self;
}

mod sealed {
    pub trait Sealed {}
}

pub trait OrNothing: sealed::Sealed {
    type Value: Output;
    type Error;

    fn or_nothing(self) -> Self::Value;
}

impl<T, E> sealed::Sealed for Result<T, E> {}

impl<T: Output, E> OrNothing for Result<T, E> {
    type Value = T;
    type Error = E;

    fn or_nothing(self) -> Self::Value {
        self.unwrap_or_else(|_| Self::Value::nothing())
    }
}

pub trait Find: Base {
    type Output: Output<Item = Self::EdgeId>;

    fn find(&self, connection: Self::Connection) -> FindResult<Self>;

    fn find_connecting(&self, one: Self::NodeId, two: Self::NodeId) -> FindResult<Self> {
        self.find(Self::Connection::connecting(one, two))
    }
}

use thiserror::Error;

use crate::connections::Connection;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum Missing<C: Connection> {
    #[error("full `{0}` is missing")]
    Full(C),
    #[error("one `{0}` is missing")]
    One(C::Id),
    #[error("two `{0}` is missing")]
    Two(C::Id),
}

//! Common functionality for implementing graphs.

pub mod connections;
#[macro_use]
pub mod control;
pub mod dfs;
pub mod err_or;
pub mod id;
pub mod index;
pub mod items;
pub mod next;
pub mod prelude;
pub mod record;
pub mod size;
pub mod time;

pub(crate) mod internals;

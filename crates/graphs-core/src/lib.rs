//! Core functionality for graphs.

#![forbid(unsafe_code)]
// #![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod adapters;
pub mod base;
pub mod build;
pub mod capacity;
pub mod connection;
#[macro_use]
pub mod control;
pub mod count;
pub mod create;
pub mod data;
pub mod degree;
pub mod direction;
pub mod empty;
pub mod freeze;
pub mod id;
pub mod identifiers;
pub mod incidence;
pub mod indexed;
pub mod item;
pub mod kind;
pub mod neighbors;
pub mod next;
pub mod recursive;
pub mod size;
pub mod time;
pub mod visit;
pub mod walk;

pub use adapters::Adapt;
pub use build::Build;
pub use capacity::{Capacities, Capacity, EdgeCapacity, NodeCapacity};
pub use connection::Connection;
pub use create::Create;
pub use direction::{Direction, Incoming, Outgoing};
pub use empty::Empty;
pub use freeze::{Freeze, Frozen};
pub use id::Id; // TODO: re-export more
pub use indexed::{EdgeCompact, EdgeIndexed, NodeCompact, NodeIndexed};
pub use item::{FromItems, Item};
pub use kind::{Directed, Kind, Kinded, Undirected};
pub use neighbors::{DirectedNeighbors, Neighbors};
pub use next::Next;
pub use visit::{Visit, Visitor};
pub use walk::{Walk, Walker};

pub(crate) mod internal;

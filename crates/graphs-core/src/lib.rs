//! Core functionality for graphs.

#![forbid(unsafe_code)]
// #![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod base;
// pub mod build;
pub mod capacity;
pub mod connection;
pub mod count;
pub mod create;
pub mod data;
pub mod direction;
pub mod empty;
pub mod freeze;
pub mod id;
pub mod incidence;
pub mod index;
// pub mod item;
pub mod kind;
pub mod neighbors;
pub mod reversed;
pub mod size;
pub mod visit;
pub mod walk;

pub use base::Base;
// pub use build::Build;
pub use capacity::{Capacities, Capacity, EdgeCapacity, NodeCapacity};
pub use connection::{Connection, Connector};
pub use create::Create;
pub use direction::{Direction, Incoming, Outgoing};
pub use empty::Empty;
pub use freeze::{Freezable, Frozen};
pub use id::{DefaultId, Id, Linked};
pub use index::{EdgeCompact, EdgeIndex, NodeCompact, NodeIndex};
// pub use item::{FromItems, Item};
pub use kind::{Directed, Kind, Kinded, Undirected};
pub use neighbors::{DirectedNeighbors, Neighbors};
pub use reversed::Reversed;
pub use visit::{Visitable, Visitor};
pub use walk::{Walk, Walker};

pub(crate) mod internal;

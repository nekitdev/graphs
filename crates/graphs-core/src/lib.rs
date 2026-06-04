//! Core functionality for graphs.

// #![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod adapters;
pub mod algorithms;
pub mod attached;
pub mod base;
pub mod build;
pub mod by;
pub mod capacity;
pub mod cardinality;
pub mod clear;
pub mod connections;
pub mod create;
pub mod data;
pub mod degree;
pub mod direction;
pub mod edges;
pub mod exhaust;
pub mod extend;
pub mod find;
pub mod freeze;
pub mod frozen;
pub mod id;
pub mod identifiers;
pub mod index;
pub mod indexed;
pub mod kinds;
pub mod loops;
pub mod map;
pub mod markers;
pub mod neighbors;
pub mod nodes;
#[macro_use]
pub mod recoverable;
pub mod references;
pub mod reserve;
pub mod reverse;
pub mod sentinel;
#[macro_use]
pub mod specs;
pub mod prelude;
pub mod subgraph;
pub mod types;
pub mod update;
pub mod visit;
pub mod walk;

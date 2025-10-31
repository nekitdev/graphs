//! Core functionality for graphs.

// #![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod adapters;
pub mod algorithm;
pub mod base;
pub mod build;
pub mod by;
pub mod capacity;
pub mod clear;
#[macro_use]
pub mod control;
pub mod connections;
pub mod count;
pub mod create;
pub mod cycles;
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
pub mod indexed;
pub mod keys;
#[macro_use]
pub mod items;
pub mod kinds;
pub mod loops;
pub mod markers;
pub mod neighbors;
// pub mod next;
#[macro_use]
pub mod recoverable;
pub mod recursive;
pub mod reverse;
pub mod select;
pub mod size;
pub mod specs;
pub mod time;
pub mod types;
pub mod visit;
pub mod walk;

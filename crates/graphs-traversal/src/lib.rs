#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod bfs;
pub mod dfs;
pub mod dfs_post_order;
pub mod topological;
pub mod traverse;

pub use bfs::Bfs;
pub use dfs::Dfs;
pub use dfs_post_order::DfsPostOrder;
pub use topological::Topological;
pub use traverse::{Traverse, TraverseTopological, TraverseTopologicalWalk, TraverseWalk};

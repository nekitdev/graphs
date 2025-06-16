//! Graph data structures and algorithms.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use cfg_if::cfg_if;

pub use graphs_core as core;

cfg_if! {
    if #[cfg(feature = "map")] {
        pub use graphs_map as map;

        pub use map::{GraphMap, DiGraphMap, UnGraphMap};
    }
}

cfg_if! {
    if #[cfg(feature = "traversal")] {
        pub use graphs_traversal as traversal;

        pub use traversal::{Dfs, DfsPostOrder, Bfs};
    }
}

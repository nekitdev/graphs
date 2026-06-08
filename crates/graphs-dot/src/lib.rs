pub use graphs_dot_parser as parser;

#[cfg(feature = "macros")]
pub use graphs_dot_macros as macros;

#[cfg(feature = "macros")]
pub use macros::{include_graph, include_graph_str};

pub mod dot;
pub mod prelude;

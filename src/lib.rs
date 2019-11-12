#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod directed_bijective_connection_graph;

pub use directed_bijective_connection_graph::functions::DirectedBijectiveConnectionGraphFunctions;
pub use directed_bijective_connection_graph::*;
pub mod graphs;
pub mod node_path;

pub type Node = u64;
pub type Dims = u64;

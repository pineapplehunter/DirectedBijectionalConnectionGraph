#![forbid(unsafe_code)]

mod directed_bijective_connection_graph;

pub use directed_bijective_connection_graph::{
    functions::DirectedBijectiveConnectionGraphFunctions, DirectedBijectiveConnectionGraph,
};
pub mod hypercube;
pub mod node_path;

pub type Node = u64;
pub type Dims = u64;

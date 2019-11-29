#![forbid(unsafe_code)]
//#![warn(missing_docs)]

mod directed_bijective_connection_graph;

pub use crate::directed_bijective_connection_graph::graph::DirectedBijectiveConnectionGraph;
pub use crate::directed_bijective_connection_graph::{Lemma1, Lemma2, NodeToNode, NodeToSet};

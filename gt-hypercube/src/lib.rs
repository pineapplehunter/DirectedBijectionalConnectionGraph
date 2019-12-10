mod hypercube;
pub use crate::hypercube::HyperCube;

pub mod graph {
    pub use gt_directed_bijective_connection_graph::{
        NPathsToNode, NodeToNodeDisjointPaths, NodeToSetDisjointPaths, SinglePath,
    };
}

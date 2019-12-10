use crate::{NPathsToNode, NodeToNodeDisjointPaths, NodeToSetDisjointPaths, SinglePath};
use gt_graph::{Dims, Graph, Node};

pub trait DirectedBijectiveConnectionGraph:
    Graph + SinglePath + NPathsToNode + NodeToNodeDisjointPaths + NodeToSetDisjointPaths
{
    #[inline(always)]
    fn psi(&self, n: Dims, node: Node) -> Node {
        self.phi(n, node)
    }
}

use crate::{Lemma1, Lemma2, NodeToNode, NodeToSet};
use gt_graph::{Dims, Graph, Node};

pub trait DirectedBijectiveConnecionGraph:
    Graph + Lemma1 + Lemma2 + NodeToNode + NodeToSet
{
    #[inline(always)]
    fn psi(&self, n: Dims, node: Node) -> Node {
        self.phi(n, node)
    }
}

use gt_graph::{Dims, Graph, Node};

pub trait DirectedBijectiveConnectionGraph: Graph {
    #[inline(always)]
    fn psi(&self, n: Dims, node: Node) -> Node {
        self.phi(n, node)
    }
}

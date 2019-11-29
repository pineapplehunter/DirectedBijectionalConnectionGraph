use gt_graph::Graph;

pub trait DirectedBijectiveConnectionGraph: Graph {
    #[inline(always)]
    fn psi(&self, n: Self::Dims, node: Self::Node) -> Self::Node {
        self.phi(n, node)
    }
}

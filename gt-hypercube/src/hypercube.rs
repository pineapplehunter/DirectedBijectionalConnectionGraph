use gt_directed_bijective_connection_graph::DirectedBijectiveConnecionGraph;
use gt_graph::{Dims, Graph, Node};

pub struct HyperCube(Dims);
impl Graph for HyperCube {
    fn dimension(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    fn phi(&self, n: Dims, node: Node) -> Node {
        node ^ (1 << (n - 1))
    }
}

impl DirectedBijectiveConnecionGraph for HyperCube {}

impl HyperCube {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

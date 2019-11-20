use gt_directed_bijective_connection_graph::DirectedBijectiveConnecionGraph;
use gt_graph::{Dims, Graph, Node};

pub struct LocallyTwistedCube(Dims);

impl LocallyTwistedCube {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

impl Graph for LocallyTwistedCube {
    fn dimension(&self) -> u64 {
        self.0
    }

    fn phi(&self, n: Dims, node: Node) -> Node {
        if n < 3 {
            node ^ (1 << (n - 1))
        } else {
            node ^ ((0b10 | (node & 1)) << (n - 2))
        }
    }
}

impl DirectedBijectiveConnecionGraph for LocallyTwistedCube {}

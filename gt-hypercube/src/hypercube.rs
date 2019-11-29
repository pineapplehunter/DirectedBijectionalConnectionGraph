use gt_directed_bijective_connection_graph::DirectedBijectiveConnecionGraph;
use gt_graph::Graph;

pub struct HyperCube(u8);
impl Graph for HyperCube {
    type Node = u64;
    type Dims = u8;

    fn dimension(&self) -> Self::Dims {
        self.0
    }

    #[inline(always)]
    fn phi(&self, n: Self::Dims, node: Self::Node) -> Self::Node {
        node ^ (1 << (n as u64 - 1))
    }
}

impl DirectedBijectiveConnecionGraph for HyperCube {}

impl HyperCube {
    pub fn new(n: u8) -> Self {
        Self(n)
    }
}

use crate::{Dims, DirectedBijectiveConnectionGraphFunctions, Node};

pub struct HyperCube(Dims);
impl DirectedBijectiveConnectionGraphFunctions for HyperCube {
    #[inline(always)]
    fn phi(&self, n: Dims, node: Node) -> Node {
        node ^ (1 << (n - 1))
    }

    fn dimension(&self) -> u64 {
        self.0
    }
}

impl HyperCube {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

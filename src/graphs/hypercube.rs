use crate::directed_bijective_connection_graph::functions::DirectedBijectiveConnectionGraphFunctions;
use crate::{Dims, Node};
use std::ops::BitXor;

pub struct HypercubeGraph(Dims);
impl DirectedBijectiveConnectionGraphFunctions for HypercubeGraph {
    #[inline(always)]
    fn phi(&self, n: Dims, node: Node) -> Node {
        (1 << (n - 1)).bitxor(node)
    }

    fn dimension(&self) -> u64 {
        self.0
    }
}

impl HypercubeGraph {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

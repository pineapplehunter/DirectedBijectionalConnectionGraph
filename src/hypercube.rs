use crate::directed_bijective_connection_graph::functions::DirectedBijectiveConnectionGraphFunctions;
use crate::directed_bijective_connection_graph::DirectedBijectiveConnectionGraph;
use crate::{Dims, Node};
use std::ops::BitXor;

impl DirectedBijectiveConnectionGraph<HypercubeFunctions> {
    pub fn new_hypercube(n: Dims) -> DirectedBijectiveConnectionGraph<HypercubeFunctions> {
        Self::new(n)
    }
}

pub struct HypercubeFunctions;
impl DirectedBijectiveConnectionGraphFunctions for HypercubeFunctions {
    #[inline(always)]
    fn phi(n: Dims, node: Node) -> Node {
        (1 << (n - 1)).bitxor(node)
    }
}

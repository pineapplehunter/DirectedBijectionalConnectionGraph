use crate::directed_bijectional_connection_graph::{
    DirectedBijectiveConnectionGraph, DirectedBijectionalConnectionGraphFunctions,
};
use crate::{Dims, Node};
use std::ops::BitXor;

impl DirectedBijectiveConnectionGraph<HypercubeFunctions> {
    pub fn new_hypercube(n: Dims) -> DirectedBijectiveConnectionGraph<HypercubeFunctions> {
        Self::new(n, HypercubeFunctions {})
    }
}

pub struct HypercubeFunctions {}
impl DirectedBijectionalConnectionGraphFunctions for HypercubeFunctions {
    #[inline(always)]
    fn phi(&self, n: Dims, node: Node) -> Node {
        (1u64 << (n - 1)).bitxor(node)
    }
}

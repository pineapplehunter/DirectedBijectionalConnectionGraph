use crate::{Dims, DirectedBijectiveConnectionGraphFunctions, Node};
use std::ops::BitXor;

pub struct LocallyTwistedCube(Dims);

impl LocallyTwistedCube {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

impl DirectedBijectiveConnectionGraphFunctions for LocallyTwistedCube {
    fn phi(&self, n: Dims, node: Node) -> Node {
        if n < 3 {
            node.bitxor(1 << (n - 1))
        } else {
            node.bitxor((0b10 | (node & 1)) << (n - 2))
        }
    }

    fn dimension(&self) -> u64 {
        self.0
    }
}

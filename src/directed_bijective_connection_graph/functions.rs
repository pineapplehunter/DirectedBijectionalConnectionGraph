use crate::{Dims, Node};

pub trait DirectedBijectiveConnectionGraphFunctions {
    fn phi(&self, n: Dims, node: Node) -> Node;

    #[inline(always)]
    fn psi(&self, n: Dims, node: Node) -> Node {
        self.phi(n, node)
    }

    fn dimension(&self) -> Dims;
}

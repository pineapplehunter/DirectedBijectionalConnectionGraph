use crate::{Dims, Node};

pub trait DirectedBijectiveConnectionGraphFunctions {
    fn phi(n: Dims, node: Node) -> Node;

    #[inline(always)]
    fn psi(n: Dims, node: Node) -> Node {
        Self::phi(n, node)
    }
}

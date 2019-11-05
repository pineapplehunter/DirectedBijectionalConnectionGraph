use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraphFunctions, Lemma2, Node,
};
use std::ops::BitXor;

fn main() {
    let n = 8;
    let s = 0b0011_0011;
    let d = 0b1010_1010;

    let graph = CustomFunctionGraph::new(n);

    let path = graph.lemma2(s, d);

    println!("{:?}", path);
}

struct CustomFunctionGraph(Dims);

impl CustomFunctionGraph {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

impl DirectedBijectiveConnectionGraphFunctions for CustomFunctionGraph {
    fn phi(&self, n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask == 0 {
            (1 << (n - 1)).bitxor(node)
        } else {
            (u64::max_value() << (n))
                .bitxor(u64::max_value())
                .bitxor(node)
        }
    }

    fn psi(&self, n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask != 0 {
            (1 << (n - 1)).bitxor(node)
        } else {
            (u64::max_value() << (n))
                .bitxor(u64::max_value())
                .bitxor(node)
        }
    }

    #[inline(always)]
    fn dimension(&self) -> u64 {
        self.0
    }
}

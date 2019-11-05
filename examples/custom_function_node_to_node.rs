use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraphFunctions, Node, NodeToNode,
};
use std::ops::BitXor;

fn main() {
    let n = 8;
    let s = 0b0101_0101;
    let d = 0b0000_1111;

    let graph = CustomFunctionGraph::new(n);

    let paths = graph.node_to_node(s, d);

    println!("{:#?}", paths);
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

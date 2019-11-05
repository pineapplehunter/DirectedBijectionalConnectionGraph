use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraphFunctions, Node, NodeToSet,
};
use std::ops::BitXor;

fn main() {
    let n = 8;
    let s = 0b0101_0101;
    let mut d = vec![];

    for i in 0..8 {
        d.push(1 << i);
    }

    let graph = CustomFunctionGraph::new(n);

    let paths = graph.node_to_set(s, &d);

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

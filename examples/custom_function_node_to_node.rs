use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraph, DirectedBijectiveConnectionGraphFunctions, Node,
};
use std::ops::BitXor;

fn main() {
    let dim = 8;

    let graph = DirectedBijectiveConnectionGraph::<Functions>::new(dim);

    let s = 0b0101_0101;
    let d = 0b0000_1111;

    let paths = graph.node_to_node(s, d);

    println!("{:#?}", paths);
}

struct Functions;

impl DirectedBijectiveConnectionGraphFunctions for Functions {
    #[inline(always)]
    fn phi(n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask == 0 {
            (1 << (n - 1)).bitxor(node)
        } else {
            (u64::max_value() << (n))
                .bitxor(u64::max_value())
                .bitxor(node)
        }
    }

    #[inline(always)]
    fn psi(n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask != 0 {
            (1 << (n - 1)).bitxor(node)
        } else {
            (u64::max_value() << (n))
                .bitxor(u64::max_value())
                .bitxor(node)
        }
    }
}

use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraph, DirectedBijectiveConnectionGraphFunctions, Node,
};
use std::ops::BitXor;

fn main() {
    let s = 0b01001101;
    let d = 0b10100101;
    let n = 8;

    let graph = DirectedBijectiveConnectionGraph::<Functions>::new(n);

    let path = graph.lemma1(n, s);
    println!("{:#?}", path);
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

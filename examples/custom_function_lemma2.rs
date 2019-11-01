use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraph, DirectedBijectiveConnectionGraphFunctions, Node,
};
use std::ops::BitXor;

fn main() {
    let graph = DirectedBijectiveConnectionGraph::<Functions>::new(8);

    let s = 0b0011_0011;
    let d = 0b1010_1010;

    let path = graph.lemma2(s, d);

    println!("path from {:08b} to {:08b} in a hyper cube.", s, d);
    println!("{:?}", path);
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

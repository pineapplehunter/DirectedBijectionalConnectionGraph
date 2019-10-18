use directed_bijectional_connection_graph::directed_bijectional_connection_graph::DirectedBijectionalConnectionGraphFunctions as GraphFunctions;
use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraph as Graph, Node,
};
use std::ops::BitXor;

fn main() {
    let s = 0b1010010101001101;
    let d = 0b0110100101010110;
    let n = 16;

    let graph = Graph::new(n, Functions);
    //let graph = Graph::new_hypercube(n);

    let path = graph.R(s, d);

    println!("{}", path);
}

struct Functions;

impl GraphFunctions for Functions {
    fn phi(&self, n: Dims, node: Node) -> Node {
        (1 << (n - 1)).bitxor(node)
    }

    fn psi(&self, n: Dims, node: Node) -> Node {
        (u64::max_value() << (n))
            .bitxor(u64::max_value())
            .bitxor(node)
    }
}

use gt_directed_bijective_connection_graph::{
    DirectedBijectiveConnectionGraph, Lemma1, Lemma2, NodeToNode, NodeToSet,
};
use gt_graph::Graph;

fn main() {
    example_lemma1();
    println!("#####################");
    example_lemma2();
    println!("#####################");
    example_node_to_set();
    println!("#####################");
    example_node_to_node();
}

fn example_lemma1() {
    println!("this is an example of lemma1 on a CustomFunctionGraph.");
    let n = 8;
    let s = 0b0000_0001;

    let graph = CustomFunctionGraph::new(n);

    let path = graph.lemma1(n, s);

    println!("{:#?}", path);
}

fn example_lemma2() {
    println!("this is an example of lemma2 on a CustomFunctionGraph");
    let n = 8;
    let s = 0b0011_0011;
    let d = 0b1010_1010;

    let graph = CustomFunctionGraph::new(n);

    let path = graph.lemma2(s, d);

    println!("{:?}", path);
}

fn example_node_to_set() {
    println!("This is an example of node to set on a CustomFunctionGraph");
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

fn example_node_to_node() {
    println!("This is an example of node to node on a CustomFunctionGraph");
    let n = 8;
    let s = 0b0101_0101;
    let d = 0b0000_1111;

    let graph = CustomFunctionGraph::new(n);

    let paths = graph.node_to_node(s, d);

    println!("{:#?}", paths);
}

#[derive(Clone, Debug)]
struct CustomFunctionGraph(u8);

impl CustomFunctionGraph {
    pub fn new(n: u8) -> Self {
        Self(n)
    }
}

impl Graph for CustomFunctionGraph {
    type Node = u8;
    type Dims = u8;

    fn dimension(&self) -> Self::Dims {
        self.0
    }

    fn phi(&self, n: Self::Dims, s: Self::Node) -> Self::Node {
        let mask = 1 << (n - 1);
        if s & mask == 0 {
            (1 << (n - 1)) ^ s
        } else {
            (u8::max_value() << n) ^ (u8::max_value()) ^ s
        }
    }
}

impl DirectedBijectiveConnectionGraph for CustomFunctionGraph {
    fn psi(&self, n: Self::Dims, node: Self::Node) -> Self::Node {
        let mask = 1 << (n - 1);
        if node & mask != 0 {
            (1 << (n - 1)) ^ (node)
        } else {
            (u8::max_value() << n) ^ (u8::max_value()) ^ node
        }
    }
}

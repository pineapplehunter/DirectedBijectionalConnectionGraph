use gt_hypercube::graph::{Lemma1, Lemma2, NodeToNode, NodeToSet};
use gt_hypercube::HyperCube;

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
    println!("this is an example of lemma1 on a HyperCube.");
    let n = 8;
    let s = 0b0000_0001;

    let graph = HyperCube::new(n);

    let path = graph.lemma1(n, s);

    println!("{:#?}", path);
}

fn example_lemma2() {
    println!("this is an example of lemma2 on a HyperCube");
    let n = 8;
    let s = 0b0011_0011;
    let d = 0b1010_1010;

    let graph = HyperCube::new(n);

    let path = graph.lemma2(s, d);

    println!("{:?}", path);
}

fn example_node_to_set() {
    println!("This is an example of node to set on a HyperCube");
    let n = 8;
    let s = 0b0101_0101;
    let mut d = vec![];

    for i in 0..8 {
        d.push(1 << i);
    }

    let graph = HyperCube::new(n);

    let paths = graph.node_to_set(s, &d);

    println!("{:#?}", paths);
}

fn example_node_to_node() {
    println!("This is an example of node to node on a HyperCube");
    let n = 8;
    let s = 0b0101_0101;
    let d = 0b0000_1111;

    let graph = HyperCube::new(n);

    let paths = graph.node_to_node(s, d);

    println!("{:#?}", paths);
}

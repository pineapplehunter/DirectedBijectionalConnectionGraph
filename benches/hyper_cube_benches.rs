#![feature(test)]

extern crate test;
use directed_bijectional_connection_graph::DirectedBijectiveConnectionGraph;
use test::Bencher;

#[bench]
fn hyper_cube_lemma1_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = DirectedBijectiveConnectionGraph::new_hypercube(dim);

    let d = 0x0000000000000000;

    b.iter(|| graph.lemma1(dim, d));
}

#[bench]
fn hyper_cube_lemma2_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = DirectedBijectiveConnectionGraph::new_hypercube(dim);

    let s = 0x0000000000000000;
    let d = 0xffffffffffffffff;

    b.iter(|| graph.lemma2(s, d));
}

#[bench]
fn hyper_cube_node_to_set_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = DirectedBijectiveConnectionGraph::new_hypercube(dim);

    let s = 0x0000000000000000;
    let mut d = vec![];

    for i in 0..64 {
        d.push(1 << i);
    }

    b.iter(|| graph.node_to_set(s, &d));
}

#[bench]
fn hyper_cube_node_to_node_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = DirectedBijectiveConnectionGraph::new_hypercube(dim);

    let s = 0x0000000000000000;
    let d = 0xffffffffffffffff;
    b.iter(|| graph.node_to_node(s, d));
}

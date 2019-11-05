#![feature(test)]

extern crate test;
use directed_bijectional_connection_graph::graphs::LocallyTwistedCube;
use directed_bijectional_connection_graph::{Lemma1, Lemma2, NodeToNode, NodeToSet};
use test::Bencher;

#[bench]
fn ltq_lemma1_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = LocallyTwistedCube::new(dim);

    let d = 0x0000_0000_0000_0000;

    b.iter(|| graph.lemma1(dim, d));
}

#[bench]
fn ltq_lemma2_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = LocallyTwistedCube::new(dim);

    let s = 0x0000_0000_0000_0000;
    let d = 0xffff_ffff_ffff_ffff;

    b.iter(|| graph.lemma2(s, d));
}

#[bench]
fn ltq_node_to_set_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = LocallyTwistedCube::new(dim);

    let s = 0x0000_0000_0000_0000;
    let mut d = vec![];

    for i in 0..64 {
        d.push(1 << i);
    }

    b.iter(|| graph.node_to_set(s, &d));
}

#[bench]
fn ltq_node_to_node_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = LocallyTwistedCube::new(dim);

    let s = 0x0000_0000_0000_0000;
    let d = 0xffff_ffff_ffff_ffff;
    b.iter(|| graph.node_to_node(s, d));
}

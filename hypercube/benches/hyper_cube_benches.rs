#![feature(test)]

extern crate test;

use hypercube::graph::{Lemma1, Lemma2, NodeToNode, NodeToSet};
use hypercube::HyperCube;
use test::Bencher;

#[bench]
fn hyper_cube_lemma1_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = HyperCube::new(dim);

    let d = 0xffff_ffff_0000_0000;

    b.iter(|| graph.lemma1(dim, d));
}

#[bench]
fn hyper_cube_lemma2_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = HyperCube::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let d = 0xffff_ffff_0000_0000;

    b.iter(|| graph.lemma2(s, d));
}

#[bench]
fn hyper_cube_node_to_set_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = HyperCube::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let mut d = vec![];

    for i in 0..64 {
        d.push(1 << i);
    }

    b.iter(|| graph.node_to_set(s, &d));
}

#[bench]
fn hyper_cube_node_to_node_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = HyperCube::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let d = 0xffff_ffff_0000_0000;
    b.iter(|| graph.node_to_node(s, d));
}

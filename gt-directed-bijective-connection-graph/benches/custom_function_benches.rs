#![feature(test)]

extern crate test;
use gt_directed_bijective_connection_graph::{
    DirectedBijectiveConnectionGraph, Lemma1, Lemma2, NodeToNode, NodeToSet,
};
use gt_graph::Graph;
use test::Bencher;

#[bench]
fn custom_function_lemma1_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let d = 0xffff_ffff_0000_0000;

    b.iter(|| graph.lemma1(dim, d));
}

#[bench]
fn custom_function_lemma2_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let d = 0xffff_ffff_0000_0000;

    b.iter(|| graph.lemma2(s, d));
}

#[bench]
fn custom_function_node_to_set_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let mut d = vec![];

    for i in 0..64 {
        d.push(1 << i);
    }

    b.iter(|| graph.node_to_set(s, &d));
}

#[bench]
fn custom_function_node_to_node_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_ffff_ffff;
    let d = 0xffff_ffff_0000_0000;
    b.iter(|| graph.node_to_node(s, d));
}

struct CustomFunctionGraph(u8);

impl CustomFunctionGraph {
    pub fn new(n: u8) -> Self {
        Self(n)
    }
}

impl Graph for CustomFunctionGraph {
    type Node = u64;
    type Dims = u8;

    #[inline(always)]
    fn dimension(&self) -> Self::Dims {
        self.0
    }

    fn phi(&self, n: Self::Dims, node: Self::Node) -> Self::Node {
        let mask = 1 << (n - 1) as u64;
        if node & mask == 0 {
            (1 << (n - 1)) as u64 ^ (node)
        } else {
            (u64::max_value() << (n)) ^ (u64::max_value()) ^ (node)
        }
    }
}

impl DirectedBijectiveConnectionGraph for CustomFunctionGraph {
    fn psi(&self, n: Self::Dims, node: Self::Node) -> Self::Node {
        let mask = 1 << (n - 1) as u64;
        if node & mask != 0 {
            (1 << (n - 1)) as u64 ^ (node)
        } else {
            (u64::max_value() << (n)) ^ (u64::max_value()) ^ (node)
        }
    }
}

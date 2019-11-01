#![feature(test)]

extern crate test;
use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraph, DirectedBijectiveConnectionGraphFunctions, Node,
};
use std::ops::BitXor;
use test::Bencher;

#[bench]
fn custom_function_lemma1_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = DirectedBijectiveConnectionGraph::<Functions>::new(dim);

    let d = 0x0000000000000000;

    b.iter(|| graph.lemma1(dim, d));
}

#[bench]
fn custom_function_lemma2_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = DirectedBijectiveConnectionGraph::<Functions>::new(dim);

    let s = 0x0000000000000000;
    let d = 0xffffffffffffffff;

    b.iter(|| graph.lemma2(s, d));
}

#[bench]
fn custom_function_node_to_set_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = DirectedBijectiveConnectionGraph::<Functions>::new(dim);

    let s = 0x0000000000000000;
    let mut d = vec![];

    for i in 0..64 {
        d.push(1 << i);
    }

    b.iter(|| graph.node_to_set(s, &d));
}

#[bench]
fn custom_function_node_to_node_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = DirectedBijectiveConnectionGraph::<Functions>::new(dim);

    let s = 0x0000000000000000;
    let d = 0xffffffffffffffff;
    b.iter(|| graph.node_to_node(s, d));
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

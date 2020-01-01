use gt_directed_bijective_connection_graph::{
    DirectedBijectiveConnectionGraph, NPathsToNode, NodeToNodeDisjointPaths,
    NodeToSetDisjointPaths, SinglePath,
};
use gt_graph::{Dims, Graph, Node};

#[test]
fn lemma1() {
    let graph = CustomFunctionGraph::new(4);
    let paths = graph.n_paths_to_node(4, 0b0011);

    assert!(paths.iter().all(|path| path.is_valid()));
    let mut deduped = paths.clone();
    deduped.dedup();
    assert_eq!(paths.len(), deduped.len());
    assert!(paths
        .iter()
        .all(|path| (path.first().unwrap() & (1 << 3)) != 0));
}

#[test]
fn lemma2() {
    let graph = CustomFunctionGraph::new(8);
    let path = graph.single_path(0b0011_0011, 0b1010_1010);

    assert!(path.is_valid());
    assert_eq!(path.first().unwrap(), &0b0011_0011);
    assert_eq!(path.last().unwrap(), &0b1010_1010);
}

#[test]
fn node_to_set() {
    let graph = CustomFunctionGraph::new(4);

    let s = 0b0000;
    let mut d = vec![];

    for i in 0..4 {
        d.push(1 << i);
    }

    let paths = graph.node_to_set_disjoint_paths(s, &d);

    assert_eq!(paths.len(), 4);
    assert!(paths
        .iter()
        .take(paths.len() - 1)
        .zip(paths.iter().skip(1))
        .all(|(p1, p2)| p1 != p2));
    paths.iter().enumerate().for_each(|(idx, path)| {
        assert!(path.is_valid());
        assert_eq!(path.first().unwrap(), &0b0000);
        assert_eq!(path.last().unwrap(), &d[idx]);
    })
}

#[test]
fn node_to_node() {
    let graph = CustomFunctionGraph::new(4);

    let s = 0b0000;
    let d = 0b1111;

    let paths = graph.node_to_node_disjoint_paths(s, d);

    assert_eq!(paths.len(), 4);
    assert!(paths
        .iter()
        .take(paths.len() - 1)
        .zip(paths.iter().skip(1))
        .all(|(p1, p2)| p1 != p2));
    paths.iter().for_each(|path| {
        assert!(path.is_valid());
        assert_eq!(path.first().unwrap(), &0b0000);
        assert_eq!(path.last().unwrap(), &0b1111);
    })
}

struct CustomFunctionGraph(Dims);

impl CustomFunctionGraph {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

impl Graph for CustomFunctionGraph {
    #[inline(always)]
    fn dimension(&self) -> u64 {
        self.0
    }

    fn phi(&self, n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask == 0 {
            (1 << (n - 1)) ^ (node)
        } else {
            (u64::max_value() << (n)) ^ (u64::max_value()) ^ (node)
        }
    }
}

impl DirectedBijectiveConnectionGraph for CustomFunctionGraph {
    fn psi(&self, n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask != 0 {
            (1 << (n - 1)) ^ (node)
        } else {
            (u64::max_value() << (n)) ^ (u64::max_value()) ^ (node)
        }
    }
}

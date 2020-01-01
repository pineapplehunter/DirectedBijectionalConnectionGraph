use crate::{DirectedBijectiveConnectionGraph, NPathsToNode, NodeToSetDisjointPaths, SinglePath};
use gt_graph::{Dims, Node};
use gt_graph_path::GraphPath;

pub trait NodeToNodeDisjointPaths {
    fn node_to_node_disjoint_paths(&self, s: Node, d: Node) -> Vec<GraphPath>;
}

impl<F> NodeToNodeDisjointPaths for F
where
    F: DirectedBijectiveConnectionGraph + SinglePath + NPathsToNode + NodeToSetDisjointPaths,
{
    fn node_to_node_disjoint_paths(&self, s: Node, d: Node) -> Vec<GraphPath> {
        node_to_node_disjoint_paths_helper(self, self.dimension(), s, d)
    }
}

fn node_to_node_disjoint_paths_helper<F>(graph: &F, n: Dims, s: Node, d: Node) -> Vec<GraphPath>
where
    F: DirectedBijectiveConnectionGraph + SinglePath + NPathsToNode + NodeToSetDisjointPaths,
{
    let mut paths;

    let mask = 1 << (n - 1);

    if s & mask == d & mask {
        paths = node_to_node_disjoint_paths_helper(graph, n - 1, s, d);

        let mut path = GraphPath::new(graph);
        path.push(s);
        let phi_s = graph.phi(n, s);
        let tmp_path = graph.single_path(phi_s, graph.psi(n, d));
        path.extend(tmp_path.iter());
        path.push(d);

        paths.push(path);
    } else {
        let mut path = GraphPath::new(graph);
        path.push(s);
        let phi_s = graph.phi(n, s);
        let tmp_path = graph.single_path(phi_s, d);
        path.extend(tmp_path.iter());

        let neighbor_node = path[path.len() - 2];

        let lemma1_except_neighbor = graph
            .n_paths_to_node(n, d)
            .into_iter()
            .filter(|path| !path.contains(&neighbor_node))
            .collect::<Vec<_>>();

        let ds = lemma1_except_neighbor
            .iter()
            .map(|path| path[0])
            .collect::<Vec<_>>();

        let mut partial_paths = graph.node_to_set_disjoint_paths(s, &ds);
        partial_paths
            .iter_mut()
            .zip(lemma1_except_neighbor.iter())
            .for_each(|(partial, lemma1)| partial.extend(lemma1.iter().skip(1)));

        paths = partial_paths;
        paths.push(path);
    }

    paths
}

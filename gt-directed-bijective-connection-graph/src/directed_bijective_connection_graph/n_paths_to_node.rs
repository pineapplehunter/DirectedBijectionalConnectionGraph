use crate::DirectedBijectiveConnectionGraph;
use gt_graph::{Dims, Node};
use gt_graph_path::GraphPath;

pub trait NPathsToNode {
    fn n_paths_to_node(&self, n: Dims, d: Node) -> Vec<GraphPath>;
}

impl<F> NPathsToNode for F
where
    F: DirectedBijectiveConnectionGraph,
{
    fn n_paths_to_node(&self, n: Dims, d: Node) -> Vec<GraphPath> {
        let mut paths = Vec::with_capacity(n as usize);

        let mut direct_path = GraphPath::new_with_initial_size(self, 2);
        direct_path.push(self.psi(n, d));
        direct_path.push(d);
        paths.push(direct_path);

        for i in 1..n {
            let mut p = GraphPath::new_with_initial_size(self, 3);
            let dd = self.psi(i, d);
            let ddd = self.psi(n, dd);

            p.push(ddd);
            p.push(dd);
            p.push(d);

            paths.push(p);
        }

        paths
    }
}

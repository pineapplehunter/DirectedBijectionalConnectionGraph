use crate::DirectedBijectiveConnectionGraph;
use gt_graph::{Dims, Node};
use gt_graph_path::GraphPath;

pub trait SinglePath {
    fn single_path(&self, s: Node, d: Node) -> GraphPath;
}

impl<F> SinglePath for F
where
    F: DirectedBijectiveConnectionGraph,
{
    fn single_path(&self, s: Node, d: Node) -> GraphPath {
        let mut path = GraphPath::new(self);
        path.push_back(s);

        single_path_helper(self, self.dimension(), s, d, &mut path);

        path
    }
}

fn single_path_helper<F>(graph: &F, n: Dims, s: Node, d: Node, path: &mut GraphPath)
where
    F: DirectedBijectiveConnectionGraph,
{
    // if same: do nothing
    if s == d {
        return;
    }

    // Step 1
    if n == 1 {
        path.push_back(s ^ 1);
        return;
    }

    // Step 2
    let mask = 1 << (n - 1);
    if s & mask == d & mask {
        single_path_helper(graph, n - 1, s, d, path);
        return;
    }

    // Step 3
    let phi_s;
    phi_s = graph.phi(n, s);
    path.push_back(phi_s);
    single_path_helper(graph, n - 1, phi_s, d, path);
}

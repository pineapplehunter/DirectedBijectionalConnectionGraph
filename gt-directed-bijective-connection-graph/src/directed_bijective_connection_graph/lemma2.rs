use crate::DirectedBijectiveConnecionGraph;
use gt_graph::{Dims, Node};
use gt_graph_path::GraphPath;

pub trait Lemma2 {
    fn lemma2(&self, s: Node, d: Node) -> GraphPath;
    #[allow(non_snake_case)]
    fn R(&self, s: Node, d: Node) -> GraphPath;
    #[allow(non_snake_case)]
    fn R_helper(&self, n: Dims, s: Node, d: Node, path: &mut GraphPath);
}

impl<F> Lemma2 for F
where
    F: DirectedBijectiveConnecionGraph,
{
    #[inline(always)]
    fn lemma2(&self, s: Node, d: Node) -> GraphPath {
        self.R(s, d)
    }

    #[allow(non_snake_case)]
    fn R(&self, s: Node, d: Node) -> GraphPath {
        let mut path = GraphPath::new(self);
        path.push_back(s);

        self.R_helper(self.dimension(), s, d, &mut path);

        path
    }

    #[allow(non_snake_case)]
    fn R_helper(&self, n: Dims, s: Node, d: Node, path: &mut GraphPath) {
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
            self.R_helper(n - 1, s, d, path);
            return;
        }

        // Step 3
        let phi_s;
        phi_s = self.phi(n, s);
        path.push_back(phi_s);
        self.R_helper(n - 1, phi_s, d, path);
    }
}

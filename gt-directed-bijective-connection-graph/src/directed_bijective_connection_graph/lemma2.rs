use crate::DirectedBijectiveConnecionGraph;
use gt_graph::Graph;
use gt_graph_path::GraphPath;
use std::ops::{BitAnd, BitXor, Sub};

pub trait Lemma2<G>
where
    G: Graph,
{
    fn lemma2(&self, s: G::Node, d: G::Node) -> GraphPath<G>;
    #[allow(non_snake_case)]
    fn R(&self, s: G::Node, d: G::Node) -> GraphPath<G>;
    #[allow(non_snake_case)]
    fn R_helper(&self, n: G::Dims, s: G::Node, d: G::Node, path: &mut GraphPath<G>);
}

impl<G, N, D> Lemma2<G> for G
where
    N: Copy + Clone + BitAnd<Output = N> + PartialEq + BitXor<Output = N> + From<usize>,
    D: Copy + Clone + Sub<usize, Output = D> + From<usize> + Into<usize> + PartialEq,
    G: DirectedBijectiveConnecionGraph + Graph<Node = N, Dims = D>,
{
    #[inline(always)]
    fn lemma2(&self, s: G::Node, d: G::Node) -> GraphPath<G> {
        self.R(s, d)
    }

    #[allow(non_snake_case)]
    fn R(&self, s: G::Node, d: G::Node) -> GraphPath<G> {
        let mut path = GraphPath::new(self);
        path.push_back(s);

        self.R_helper(self.dimension(), s, d, &mut path);

        path
    }

    #[allow(non_snake_case)]
    fn R_helper(&self, n: G::Dims, s: G::Node, d: G::Node, path: &mut GraphPath<G>) {
        // if same: do nothing
        if s == d {
            return;
        }

        // Step 1
        if n == 1.into() {
            path.push_back(s ^ 1.into());
            return;
        }

        // Step 2
        let mask: N = (1 << (n - 1).into()).into();
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

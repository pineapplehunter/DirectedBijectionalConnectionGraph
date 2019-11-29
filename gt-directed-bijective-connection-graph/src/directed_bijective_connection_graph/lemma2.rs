use crate::DirectedBijectiveConnectionGraph;
use gt_graph::{Graph, InterChangeUsize};
use gt_graph_path::GraphPath;
use std::ops::{BitAnd, BitXor, Sub};
use std::process::Output;

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
    N: Copy + BitAnd<Output = N> + PartialEq + BitXor<Output = N> + InterChangeUsize,
    D: Copy + InterChangeUsize + PartialEq + Sub<Output = D>,
    G: DirectedBijectiveConnectionGraph + Graph<Node = N, Dims = D>,
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
        if n == InterChangeUsize::from_usize(1) {
            path.push_back(s ^ InterChangeUsize::from_usize(1));
            return;
        }

        // Step 2
        let mask: N = InterChangeUsize::from_usize(1 << (n.to_usize() - 1));
        if s & mask == d & mask {
            self.R_helper(n - InterChangeUsize::from_usize(1), s, d, path);
            return;
        }

        // Step 3
        let phi_s;
        phi_s = self.phi(n, s);
        path.push_back(phi_s);
        self.R_helper(
            InterChangeUsize::from_usize(n.to_usize() - 1),
            phi_s,
            d,
            path,
        );
    }
}

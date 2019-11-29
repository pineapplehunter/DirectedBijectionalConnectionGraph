use crate::{DirectedBijectiveConnectionGraph, Lemma1, Lemma2, NodeToSet};
use gt_graph::Graph;
use gt_graph_path::GraphPath;
use std::ops::{BitAnd, Sub};

pub trait NodeToNode<G>
where
    G: Graph,
{
    #[allow(non_snake_case)]
    fn N2N(&self, s: G::Node, d: G::Node) -> Vec<GraphPath<G>>;
    fn node_to_node(&self, s: G::Node, d: G::Node) -> Vec<GraphPath<G>>;
    fn node_to_node_helper(&self, n: G::Dims, s: G::Node, d: G::Node) -> Vec<GraphPath<G>>;
}

impl<G, N, D> NodeToNode<G> for G
where
    N: Copy + PartialEq + BitAnd<Output = N> + From<usize>,
    D: Copy + Into<usize> + From<usize> + Sub<usize, Output = D>,
    G: DirectedBijectiveConnectionGraph
        + Lemma2<G>
        + Lemma1<G>
        + NodeToSet<G>
        + Graph<Node = N, Dims = D>,
{
    #[allow(non_snake_case)]
    #[inline(always)]
    fn N2N(&self, s: G::Node, d: G::Node) -> Vec<GraphPath<G>> {
        self.node_to_node(s, d)
    }

    #[inline(always)]
    fn node_to_node(&self, s: G::Node, d: G::Node) -> Vec<GraphPath<G>> {
        self.node_to_node_helper(self.dimension(), s, d)
    }

    fn node_to_node_helper(&self, n: G::Dims, s: G::Node, d: G::Node) -> Vec<GraphPath<G>> {
        let mut paths;

        let mask: N = (1 << (n - 1).into()).into();

        if s & mask == d & mask {
            paths = self.node_to_node_helper((n.into() - 1).into(), s, d);

            let mut path = GraphPath::new(self);
            path.push_back(s);
            let phi_s = self.phi(n, s);
            path.push_back(phi_s);
            self.R_helper(n, phi_s, self.psi(n, d), &mut path);
            path.push_back(d);

            paths.push(path);
        } else {
            let mut path = GraphPath::new(self);
            path.push_back(s);
            let phi_s = self.phi(n, s);
            path.push_back(phi_s);
            self.R_helper(n, phi_s, d, &mut path);

            let neighbor_node = path.inner_path()[path.inner_path().len() - 2];

            let lemma1_except_neighbor = self
                .lemma1(n, d)
                .into_iter()
                .filter(|path| !path.inner_path().contains(&neighbor_node))
                .collect::<Vec<_>>();

            let ds = lemma1_except_neighbor
                .iter()
                .map(|path| path.inner_path()[0])
                .collect::<Vec<_>>();

            let mut partial_paths = self.node_to_set(s, &ds);
            partial_paths
                .iter_mut()
                .zip(lemma1_except_neighbor.iter())
                .for_each(|(partial, lemma1)| {
                    partial
                        .inner_path_mut()
                        .extend(lemma1.inner_path().iter().skip(1))
                });

            paths = partial_paths;
            paths.push(path);
        }

        paths
    }
}

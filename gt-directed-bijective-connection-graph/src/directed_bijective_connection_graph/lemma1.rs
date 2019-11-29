use crate::DirectedBijectiveConnecionGraph;
use gt_graph::Graph;
use gt_graph_path::GraphPath;

pub trait Lemma1<G>
where
    G: Graph,
{
    fn lemma1(&self, n: G::Dims, d: G::Node) -> Vec<GraphPath<G>>;
}

impl<G, N, D> Lemma1<G> for G
where
    N: Copy + Clone,
    D: Copy + Clone + Into<usize> + From<usize>,
    G: DirectedBijectiveConnecionGraph + Graph<Node = N, Dims = D>,
{
    fn lemma1(&self, n: G::Dims, d: G::Node) -> Vec<GraphPath<G>> {
        let mut paths = Vec::with_capacity(n.into());

        let mut direct_path = GraphPath::new_with_initial_size(self, 2);
        direct_path.push_back(self.psi(n, d));
        direct_path.push_back(d);
        paths.push(direct_path);

        for i in 1..n.into() {
            let mut p = GraphPath::new_with_initial_size(self, 3);
            let dd = self.psi(i.into(), d);
            let ddd = self.psi(n, dd);

            p.push_back(ddd);
            p.push_back(dd);
            p.push_back(d);

            paths.push(p);
        }

        paths
    }
}

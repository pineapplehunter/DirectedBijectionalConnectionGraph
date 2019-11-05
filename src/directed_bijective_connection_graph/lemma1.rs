use crate::node_path::NodePath;
use crate::{Dims, DirectedBijectiveConnectionGraphFunctions, Node};

pub trait Lemma1 {
    fn lemma1(&self, n: Dims, d: Node) -> Vec<NodePath>;
}

impl<F> Lemma1 for F
where
    F: DirectedBijectiveConnectionGraphFunctions,
{
    fn lemma1(&self, n: Dims, d: Node) -> Vec<NodePath> {
        let mut paths = Vec::new();

        let mut direct_path = NodePath::new(self.dimension());
        direct_path.push_front(d);
        direct_path.push_front(self.psi(n, d));
        paths.push(direct_path);

        for i in 1..n {
            let mut p = NodePath::new(self.dimension());
            p.push_front(d);
            let dd = self.psi(i, d);
            let ddd = self.psi(n, dd);
            p.push_front(dd);
            p.push_front(ddd);

            paths.push(p);
        }

        paths
    }
}

#[cfg(test)]
mod test {
    use crate::hypercube::HypercubeGraph;
    use crate::node_path::NodePath;
    use crate::Lemma1;

    #[test]
    fn lemma1() {
        let graph = HypercubeGraph::new(4);
        let paths = graph.lemma1(4, 0b0011);

        let expected_paths: Vec<NodePath> = vec![
            NodePath::from_vec(4, vec![0b1011, 0b0011]),
            NodePath::from_vec(4, vec![0b1010, 0b0010, 0b0011]),
            NodePath::from_vec(4, vec![0b1001, 0b0001, 0b0011]),
            NodePath::from_vec(4, vec![0b1111, 0b0111, 0b0011]),
        ];

        assert_eq!(paths, expected_paths);
    }
}

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
        let mut paths = Vec::with_capacity(n as usize);

        let mut direct_path = NodePath::new_with_initial_size(self.dimension(), 2);
        direct_path.push_back(self.psi(n, d));
        direct_path.push_back(d);
        paths.push(direct_path);

        for i in 1..n {
            let mut p = NodePath::new_with_initial_size(self.dimension(), 3);
            let dd = self.psi(i, d);
            let ddd = self.psi(n, dd);

            p.push_back(ddd);
            p.push_back(dd);
            p.push_back(d);

            paths.push(p);
        }

        paths
    }
}

#[cfg(test)]
mod test {
    use crate::graphs::HyperCube;
    use crate::node_path::NodePath;
    use crate::Lemma1;

    #[test]
    fn lemma1() {
        let graph = HyperCube::new(4);
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

use crate::node_path::NodePath;
use crate::{
    Dims, DirectedBijectiveConnectionGraph, DirectedBijectiveConnectionGraphFunctions, Node,
};

impl<F> DirectedBijectiveConnectionGraph<F>
where
    F: DirectedBijectiveConnectionGraphFunctions,
{
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn N2N(&self, s: Node, d: Node) -> Vec<NodePath> {
        self.node_to_node(s, d)
    }

    #[inline(always)]
    pub fn node_to_node(&self, s: Node, d: Node) -> Vec<NodePath> {
        self.node_to_node_helper(self.dimension, s, d)
    }

    pub fn node_to_node_helper(&self, n: Dims, s: Node, d: Node) -> Vec<NodePath> {
        let mut paths;

        let mask = 1 << (n - 1);

        if s & mask == d & mask {
            paths = self.node_to_node_helper(n - 1, s, d);

            let mut path = NodePath::new(self.dimension);
            path.push_back(s);
            let phi_s = F::phi(n, s);
            path.push_back(phi_s);
            self.R_helper(n, phi_s, F::psi(n, d), &mut path);
            path.push_back(d);

            paths.push(path);
        } else {
            let mut path = NodePath::new(self.dimension);
            path.push_back(s);
            let phi_s = F::phi(n, s);
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

#[cfg(test)]
mod test {
    use crate::node_path::NodePath;
    use crate::DirectedBijectiveConnectionGraph;

    #[test]
    fn node_to_set() {
        let graph = DirectedBijectiveConnectionGraph::new_hypercube(4);

        let s = 0b0000;
        let d = 0b1111;

        let paths = graph.node_to_node(s, d);

        let expected_paths: Vec<NodePath> = vec![
            NodePath::from_vec(4, vec![0b0000, 0b0100, 0b0110, 0b0111, 0b1111]),
            NodePath::from_vec(4, vec![0b0000, 0b0001, 0b0101, 0b1101, 0b1111]),
            NodePath::from_vec(4, vec![0b0000, 0b0010, 0b0011, 0b1011, 0b1111]),
            NodePath::from_vec(4, vec![0b0000, 0b1000, 0b1100, 0b1110, 0b1111]),
        ];

        assert_eq!(paths, expected_paths);
    }
}

use crate::directed_bijective_connection_graph::functions::DirectedBijectiveConnectionGraphFunctions;
use crate::directed_bijective_connection_graph::DirectedBijectiveConnectionGraph;
use crate::node_path::NodePath;
use crate::{Dims, Node};
use std::ops::BitXor;

impl<F> DirectedBijectiveConnectionGraph<F>
where
    F: DirectedBijectiveConnectionGraphFunctions,
{
    #[inline(always)]
    pub fn lemma2(&self, s_source: Node, d_dest: Node) -> NodePath {
        self.R(s_source, d_dest)
    }

    #[allow(non_snake_case)]
    pub fn R(&self, s_source: Node, d_dest: Node) -> NodePath {
        let mut path = NodePath::new(self.dimension);
        path.push_back(s_source);

        self.R_helper(self.dimension, s_source, d_dest, &mut path);

        path
    }

    #[allow(non_snake_case)]
    pub(crate) fn R_helper(&self, n_dim: Dims, s_source: Node, d_dest: Node, path: &mut NodePath) {
        // if same: do nothing
        if s_source == d_dest {
            return;
        }

        // Step 1
        if n_dim == 1 {
            path.push_back(s_source.bitxor(1));
            return;
        }

        // Step 2
        let mask = 1 << (n_dim - 1);
        if s_source & mask == d_dest & mask {
            self.R_helper(n_dim - 1, s_source, d_dest, path);
            return;
        }

        // Step 3
        let phi_s;
        phi_s = F::phi(n_dim, s_source);
        path.push_back(phi_s);
        self.R_helper(n_dim - 1, phi_s, d_dest, path);
    }
}

#[cfg(test)]
mod test {
    use crate::node_path::NodePath;
    use crate::DirectedBijectiveConnectionGraph;

    #[test]
    fn lemma2() {
        let graph = DirectedBijectiveConnectionGraph::new_hypercube(8);
        let path = graph.lemma2(0b00110011, 0b10101010);

        let expected_path = NodePath::from_vec(
            8,
            vec![0b00110011, 0b10110011, 0b10100011, 0b10101011, 0b10101010],
        );

        assert_eq!(path, expected_path);
    }
}

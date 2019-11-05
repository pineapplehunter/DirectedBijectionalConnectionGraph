use crate::directed_bijective_connection_graph::functions::DirectedBijectiveConnectionGraphFunctions;
use crate::node_path::NodePath;
use crate::{Dims, Node};
use std::ops::BitXor;

pub trait Lemma2 {
    fn lemma2(&self, s: Node, d: Node) -> NodePath;
    #[allow(non_snake_case)]
    fn R(&self, s: Node, d: Node) -> NodePath;
    #[allow(non_snake_case)]
    fn R_helper(&self, n: Dims, s: Node, d: Node, path: &mut NodePath);
}

impl<F> Lemma2 for F
where
    F: DirectedBijectiveConnectionGraphFunctions,
{
    #[inline(always)]
    fn lemma2(&self, s: Node, d: Node) -> NodePath {
        self.R(s, d)
    }

    #[allow(non_snake_case)]
    fn R(&self, s: Node, d: Node) -> NodePath {
        let mut path = NodePath::new(self.dimension());
        path.push_back(s);

        self.R_helper(self.dimension(), s, d, &mut path);

        path
    }

    #[allow(non_snake_case)]
    fn R_helper(&self, n: Dims, s: Node, d: Node, path: &mut NodePath) {
        // if same: do nothing
        if s == d {
            return;
        }

        // Step 1
        if n == 1 {
            path.push_back(s.bitxor(1));
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

#[cfg(test)]
mod test {
    use crate::graphs::HypercubeGraph;
    use crate::node_path::NodePath;
    use crate::Lemma2;

    #[test]
    fn lemma2() {
        let graph = HypercubeGraph::new(8);
        let path = graph.lemma2(0b0011_0011, 0b1010_1010);

        let expected_path = NodePath::from_vec(
            8,
            vec![
                0b0011_0011,
                0b1011_0011,
                0b1010_0011,
                0b1010_1011,
                0b1010_1010,
            ],
        );

        assert_eq!(path, expected_path);
    }
}

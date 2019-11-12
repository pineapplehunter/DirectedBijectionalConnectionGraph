use crate::directed_bijective_connection_graph::lemma1::Lemma1;
use crate::directed_bijective_connection_graph::lemma2::Lemma2;
use crate::directed_bijective_connection_graph::node_to_set::NodeToSet;
use crate::node_path::NodePath;
use crate::{Dims, DirectedBijectiveConnectionGraphFunctions, Node};

pub trait NodeToNode {
    #[allow(non_snake_case)]
    fn N2N(&self, s: Node, d: Node) -> Vec<NodePath>;
    fn node_to_node(&self, s: Node, d: Node) -> Vec<NodePath>;
    fn node_to_node_helper(&self, n: Dims, s: Node, d: Node) -> Vec<NodePath>;
}

impl<F> NodeToNode for F
where
    F: DirectedBijectiveConnectionGraphFunctions + Lemma2 + Lemma1 + NodeToSet,
{
    #[allow(non_snake_case)]
    #[inline(always)]
    fn N2N(&self, s: Node, d: Node) -> Vec<NodePath> {
        self.node_to_node(s, d)
    }

    #[inline(always)]
    fn node_to_node(&self, s: Node, d: Node) -> Vec<NodePath> {
        self.node_to_node_helper(self.dimension(), s, d)
    }

    fn node_to_node_helper(&self, n: Dims, s: Node, d: Node) -> Vec<NodePath> {
        let mut paths;

        let mask = 1 << (n - 1);

        if s & mask == d & mask {
            paths = self.node_to_node_helper(n - 1, s, d);

            let mut path = NodePath::new(self);
            path.push_back(s);
            let phi_s = self.phi(n, s);
            path.push_back(phi_s);
            self.R_helper(n, phi_s, self.psi(n, d), &mut path);
            path.push_back(d);

            paths.push(path);
        } else {
            let mut path = NodePath::new(self);
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

#[cfg(test)]
mod test {
    use crate::graphs::HyperCube;
    use crate::NodeToNode;

    #[test]
    fn node_to_set() {
        let graph = HyperCube::new(4);

        let s = 0b0000;
        let d = 0b1111;

        let paths = graph.node_to_node(s, d);

        assert_eq!(paths.len(), 4);
        assert!(paths
            .iter()
            .take(paths.len() - 1)
            .zip(paths.iter().skip(1))
            .all(|(p1, p2)| p1 != p2));
        paths.iter().for_each(|path| {
            assert!(path.is_valid());
            assert_eq!(path.inner_path().first().unwrap(), &0b0000);
            assert_eq!(path.inner_path().last().unwrap(), &0b1111);
        })
    }
}

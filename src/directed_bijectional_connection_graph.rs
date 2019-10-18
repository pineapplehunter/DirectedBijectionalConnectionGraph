use crate::node_path::NodePath;
use crate::{Dims, Node};
use std::ops::BitXor;

pub struct DirectedBijectiveConnectionGraph<F: DirectedBijectionalConnectionGraphFunctions> {
    dimension: Dims,
    functions: F,
}

pub trait DirectedBijectionalConnectionGraphFunctions {
    fn phi(&self, n: Dims, node: Node) -> Node;

    #[inline(always)]
    fn phi_inverse(&self, n: Dims, node: Node) -> Node {
        self.phi(n, node)
    }

    #[inline(always)]
    fn psi(&self, n: Dims, node: Node) -> Node {
        self.phi(n, node)
    }

    #[inline(always)]
    fn psi_inverse(&self, n: Dims, node: Node) -> Node {
        self.psi(n, node)
    }
}

impl<F> DirectedBijectiveConnectionGraph<F>
where
    F: DirectedBijectionalConnectionGraphFunctions,
{
    pub fn new(n_dim: Dims, functions: F) -> Self {
        assert_ne!(n_dim, 0);
        assert!(n_dim < 64);

        DirectedBijectiveConnectionGraph {
            dimension: n_dim,
            functions,
        }
    }

    #[allow(non_snake_case)]
    pub fn R(&self, s_source: Node, d_dest: Node) -> NodePath {
        let mut path = NodePath::new(self.dimension);
        path.push_back(s_source);

        self.R_helper(self.dimension, s_source, d_dest, &mut path);

        path
    }

    #[allow(non_snake_case)]
    fn R_helper(&self, n_dim: Dims, s_source: Node, d_dest: Node, path: &mut NodePath) {
        if s_source == d_dest {
            return;
        }

        if n_dim == 1 {
            path.push_back(s_source.bitxor(1));
            return;
        }

        let mask = 1 << (n_dim - 1);
        if s_source & mask == d_dest & mask {
            self.R_helper(n_dim - 1, s_source, d_dest, path);
            return;
        }
        let phi_s;
        if s_source & mask == 0 {
            phi_s = self.functions.phi(n_dim, s_source);
        } else {
            phi_s = self.functions.psi(n_dim, s_source);
        }
        path.push_back(phi_s);
        self.R_helper(n_dim - 1, phi_s, d_dest, path);
    }
}

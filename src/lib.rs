use std::collections::VecDeque;
use std::fmt::{Display, Error, Formatter};
use std::ops::BitXor;

trait GraphMoveFunctions {
    fn phi(n_dim: u64, node: Node) -> Node;
    fn phi_inverse(n_dim: u64, node: Node) -> Node;
    fn psi(n_dim: u64, node: Node) -> Node;
    fn psi_inverse(n_dim: u64, node: Node) -> Node;
}

struct HyperCubeFunctions;
impl GraphMoveFunctions for HyperCubeFunctions {
    #[inline(always)]
    fn phi(n_dim: u64, node: u64) -> u64 {
        (1u64 << (n_dim - 1)).bitxor(node)
    }

    #[inline(always)]
    fn phi_inverse(n_dim: u64, node: u64) -> u64 {
        HyperCubeFunctions::phi(n_dim, node)
    }

    #[inline(always)]
    fn psi(n_dim: u64, node: u64) -> u64 {
        HyperCubeFunctions::phi(n_dim, node)
    }

    fn psi_inverse(n_dim: u64, node: u64) -> u64 {
        HyperCubeFunctions::phi(n_dim, node)
    }
}

pub struct Graph {
    dimension: u64,
}

type Node = u64;

impl Graph {
    pub fn new_hypercube(n_dim: u64) -> Self {
        assert_ne!(n_dim, 0);
        assert!(n_dim < 64);

        Self { dimension: n_dim }
    }

    fn phi(n_dim: u64, node: Node) -> Node {
        (1u64 << (n_dim - 1)).bitxor(node)
    }

    #[allow(non_snake_case)]
    pub fn R(&self, s_source: Node, d_dest: Node) -> NodePath {
        let mut path = NodePath::new();
        path.push_back(s_source);

        self.R_helper(self.dimension, s_source, d_dest, &mut path);

        path
    }

    #[allow(non_snake_case)]
    fn R_helper(&self, n_dim: u64, s_source: Node, d_dest: Node, path: &mut NodePath) {
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

        let psi_s = Graph::phi(n_dim, s_source);
        path.push_back(psi_s);
        self.R_helper(n_dim - 1, psi_s, d_dest, path);
    }
}
#[derive(Debug)]
pub struct NodePath {
    path: VecDeque<Node>,
}

impl NodePath {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            path: VecDeque::new(),
        }
    }

    #[inline(always)]
    pub fn push_back(&mut self, n: Node) {
        self.path.push_back(n);
    }

    #[inline(always)]
    pub fn push_front(&mut self, n: Node) {
        self.path.push_front(n);
    }
}

impl Display for NodePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut s = String::new();
        if self.path.len() != 0 {
            s.push_str(&format!("{:04b}", self.path[0]));
            for p in self.path.iter().skip(1) {
                s.push_str(&format!(" -> {:04b}", p));
            }
        }
        write!(f, "NodePath {{ {} }}", s)
    }
}

#[cfg(test)]
mod test {
    use crate::psi;

    #[test]
    fn psi_flip() {
        let s = 0;
        let n_dim = 1;
        assert_eq!(1, psi(n_dim, s));
    }
}

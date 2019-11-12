use crate::{Dims, Node};
use std::fmt::{Debug, Error, Formatter};

#[derive(PartialEq)]
pub struct NodePath {
    dimensions: Dims,
    path: Vec<Node>,
}

impl NodePath {
    #[inline(always)]
    pub fn new(n: Dims) -> Self {
        Self {
            dimensions: n,
            path: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn new_with_initial_size(n: Dims, size: usize) -> Self {
        Self {
            dimensions: n,
            path: Vec::with_capacity(size),
        }
    }

    pub fn from_vec(n: Dims, src: Vec<Node>) -> Self {
        Self {
            dimensions: n,
            path: src,
        }
    }

    #[inline(always)]
    pub fn push_back(&mut self, n: Node) {
        self.path.push(n);
    }

    pub fn inner_path(&self) -> &Vec<Node> {
        &self.path
    }

    pub fn inner_path_mut(&mut self) -> &mut Vec<Node> {
        &mut self.path
    }
}

impl Debug for NodePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut s = String::new();
        if !self.path.is_empty() {
            s.push_str(&format!(
                "{:0dim$b}",
                self.path[0],
                dim = self.dimensions as usize
            ));
            for p in self.path.iter().skip(1) {
                s.push_str(&format!(" -> {:0dim$b}", p, dim = self.dimensions as usize));
            }
        }
        write!(f, "NodePath {{ {} }}", s)
    }
}

use crate::{Dims, Node};
use std::collections::VecDeque;
use std::fmt::{Debug, Error, Formatter};

#[derive(PartialEq)]
pub struct NodePath {
    dimensions: Dims,
    path: VecDeque<Node>,
}

impl NodePath {
    #[inline(always)]
    pub fn new(n: Dims) -> Self {
        Self {
            dimensions: n,
            path: VecDeque::new(),
        }
    }

    pub fn from_vec(n: Dims, src: Vec<Node>) -> Self {
        Self {
            dimensions: n,
            path: src.into(),
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

    pub fn inner_path(&self) -> &VecDeque<Node> {
        &self.path
    }

    pub fn inner_path_mut(&mut self) -> &mut VecDeque<Node> {
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

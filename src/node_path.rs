use crate::{Dims, Node};
use std::collections::VecDeque;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
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

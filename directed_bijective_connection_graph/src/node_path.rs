use crate::{DirectedBijectiveConnectionGraphFunctions, Node};
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone)]
pub struct NodePath<'a> {
    graph: &'a dyn DirectedBijectiveConnectionGraphFunctions,
    path: Vec<Node>,
}

impl<'a> PartialEq for NodePath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<'a> NodePath<'a> {
    #[inline(always)]
    pub fn new(graph: &'a dyn DirectedBijectiveConnectionGraphFunctions) -> Self {
        Self {
            graph,
            path: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn new_with_initial_size(
        graph: &'a dyn DirectedBijectiveConnectionGraphFunctions,
        size: usize,
    ) -> Self {
        Self {
            graph,
            path: Vec::with_capacity(size),
        }
    }

    pub fn from_vec(
        graph: &'a dyn DirectedBijectiveConnectionGraphFunctions,
        src: Vec<Node>,
    ) -> Self {
        Self { graph, path: src }
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

    pub fn is_valid(&self) -> bool {
        self.path
            .iter()
            .take(self.path.len() - 1)
            .zip(self.path.iter().skip(1))
            .all(|(&first, &second)| {
                (1..=self.graph.dimension()).any(|n| self.graph.phi(n, first) == second)
            })
    }
}

impl<'a> Debug for NodePath<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut s = String::new();
        if !self.path.is_empty() {
            s.push_str(&format!(
                "{:0dim$b}",
                self.path[0],
                dim = self.graph.dimension() as usize
            ));
            for p in self.path.iter().skip(1) {
                s.push_str(&format!(
                    " -> {:0dim$b}",
                    p,
                    dim = self.graph.dimension() as usize
                ));
            }
        }
        write!(f, "NodePath {{ {} }}", s)
    }
}

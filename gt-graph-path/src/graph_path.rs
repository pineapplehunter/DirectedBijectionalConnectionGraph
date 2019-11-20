use gt_graph::{Graph, Node};
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone)]
pub struct GraphPath<'a> {
    graph: &'a dyn Graph,
    path: Vec<Node>,
}

impl<'a> PartialEq for GraphPath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<'a> GraphPath<'a> {
    #[inline(always)]
    pub fn new(graph: &'a dyn Graph) -> Self {
        Self {
            graph,
            path: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn new_with_initial_size(graph: &'a dyn Graph, size: usize) -> Self {
        Self {
            graph,
            path: Vec::with_capacity(size),
        }
    }

    pub fn from_vec(graph: &'a dyn Graph, src: Vec<Node>) -> Self {
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

impl<'a> Debug for GraphPath<'a> {
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

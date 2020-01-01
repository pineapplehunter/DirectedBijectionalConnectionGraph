use gt_graph::{Graph, Node};
use std::fmt::{Debug, Error, Formatter};
use std::ops::{Deref, DerefMut};
use std::path::Path;

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
    pub fn push_validate(&mut self, n: Node) -> Result<(), String> {
        if self.path.last().is_none()
            || (1..=self.graph.dimension())
                .any(|n| self.graph.phi(n, *self.path.last().unwrap()) == n)
        {
            self.path.push(n);
            Ok(())
        } else {
            Err("Invalid Path".into())
        }
    }

    pub fn is_valid(&self) -> bool {
        (0..self.path.len() - 1).all(|i| {
            (1..=self.graph.dimension())
                .any(|n| self.graph.phi(n, self.path[i]) == self.path[i + 1])
        })
    }
}

impl<'a> Deref for GraphPath<'a> {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl<'a> DerefMut for GraphPath<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
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

trait Paths {
    fn contains(&self, n: Node) -> bool;
}

impl<'a> Paths for Vec<GraphPath<'a>> {
    fn contains(&self, n: Node) -> bool {
        self.iter().any(|path| path.contains(&n))
    }
}

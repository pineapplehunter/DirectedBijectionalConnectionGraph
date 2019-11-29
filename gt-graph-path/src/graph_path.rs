use gt_graph::Graph;
use std::fmt::{Binary, Debug, Error, Formatter};

#[derive(Clone)]
pub struct GraphPath<'a, G: Graph> {
    graph: &'a G,
    path: Vec<G::Node>,
}

impl<'a, G, N> PartialEq for GraphPath<'a, G>
where
    N: PartialEq,
    G: Graph<Node = N>,
{
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<'a, G: Graph> GraphPath<'a, G> {
    #[inline(always)]
    pub fn new(graph: &'a G) -> Self {
        Self {
            graph,
            path: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn new_with_initial_size(graph: &'a G, size: usize) -> Self {
        Self {
            graph,
            path: Vec::with_capacity(size),
        }
    }

    pub fn from_vec(graph: &'a G, src: Vec<G::Node>) -> Self {
        Self { graph, path: src }
    }

    #[inline(always)]
    pub fn push_back(&mut self, n: G::Node) {
        self.path.push(n);
    }

    pub fn inner_path(&self) -> &Vec<G::Node> {
        &self.path
    }

    pub fn inner_path_mut(&mut self) -> &mut Vec<G::Node> {
        &mut self.path
    }
}

impl<'a, G> GraphPath<'a, G>
where
    G: Graph<Node = u64, Dims = u64>,
{
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

impl<'a, G, N, D> Debug for GraphPath<'a, G>
where
    N: PartialEq + Binary,
    D: Into<usize>,
    G: Graph<Node = N, Dims = D>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut s = String::new();
        if !self.path.is_empty() {
            s.push_str(&format!(
                "{:0dim$b}",
                self.path[0],
                dim = self.graph.dimension().into()
            ));
            for p in self.path.iter().skip(1) {
                s.push_str(&format!(
                    " -> {:0dim$b}",
                    p,
                    dim = self.graph.dimension().into()
                ));
            }
        }
        write!(f, "NodePath {{ {} }}", s)
    }
}

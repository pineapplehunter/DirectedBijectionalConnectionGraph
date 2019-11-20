pub trait Graph {
    fn dimension(&self) -> Dims;
    fn phi(&self, n: Dims, s: Node) -> Node;
}

pub type Node = u64;
pub type Dims = u64;

pub trait Graph {
    type Node;
    type Dims;
    fn dimension(&self) -> Self::Dims;
    fn phi(&self, n: Self::Dims, s: Self::Node) -> Self::Node;
}

mod inter_change_usize;
pub use inter_change_usize::InterChangeUsize;

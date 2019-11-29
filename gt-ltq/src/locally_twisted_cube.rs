use gt_directed_bijective_connection_graph::DirectedBijectiveConnecionGraph;
use gt_graph::Graph;

/// # Locally Twisted Cube
/// an implementation of a Locally Twisted Cube.
/// ## algorithm
/// you can see more at phi() in the implementation of Graph
/// ## example
/// ```
/// use gt_ltq::LocallyTwistedCube;
/// use gt_ltq::graph::NodeToNode;
///
/// let graph = LocallyTwistedCube::new(8);
///
/// let paths = graph.node_to_node(0x00, 0xff);
/// ```
pub struct LocallyTwistedCube(u8);

impl LocallyTwistedCube {
    pub fn new(n: u8) -> Self {
        Self(n)
    }
}

impl Graph for LocallyTwistedCube {
    type Node = u64;
    type Dims = u8;

    fn dimension(&self) -> Self::Dims {
        self.0
    }

    fn phi(&self, n: Self::Dims, node: Self::Node) -> Self::Node {
        if n < 3 {
            node ^ (1 << (n as u64 - 1))
        } else {
            node ^ ((0b10 | (node & 1)) << (n as u64 - 2))
        }
    }
}

impl DirectedBijectiveConnecionGraph for LocallyTwistedCube {}

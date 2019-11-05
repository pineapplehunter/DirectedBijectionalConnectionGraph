use directed_bijectional_connection_graph::hypercube::HypercubeGraph;
use directed_bijectional_connection_graph::NodeToNode;

fn main() {
    let n = 8;
    let s = 0b0101_0101;
    let d = 0b0000_1111;

    let graph = HypercubeGraph::new(n);

    let paths = graph.node_to_node(s, d);

    println!("{:#?}", paths);
}

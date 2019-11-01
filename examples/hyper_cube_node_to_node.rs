use directed_bijectional_connection_graph::DirectedBijectiveConnectionGraph;

fn main() {
    let dim = 8;

    let graph = DirectedBijectiveConnectionGraph::new_hypercube(dim);

    let s = 0b0101_0101;
    let d = 0b0000_1111;

    let paths = graph.node_to_node(s, d);

    println!("{:#?}", paths);
}

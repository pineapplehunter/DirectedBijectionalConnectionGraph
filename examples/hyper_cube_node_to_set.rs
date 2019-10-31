use directed_bijectional_connection_graph::DirectedBijectiveConnectionGraph;

fn main() {
    let dim = 8;

    let graph = DirectedBijectiveConnectionGraph::new_hypercube(dim);

    let s = 0b01010101;
    let mut d = vec![];

    for i in 0..8 {
        d.push(1 << i);
    }

    let paths = graph.node_to_set(s, &d);

    println!("{:#?}", paths);
}

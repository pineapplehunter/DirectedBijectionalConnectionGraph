use directed_bijectional_connection_graph::DirectedBijectiveConnectionGraph;

fn main() {
    let s = 0b01001101;
    let n = 8;

    let graph = DirectedBijectiveConnectionGraph::new_hypercube(n);

    let path = graph.lemma1(n, s);

    println!("{:#?}", path);

    let s = 0b01010101;
    let d = 0b00001111;
}

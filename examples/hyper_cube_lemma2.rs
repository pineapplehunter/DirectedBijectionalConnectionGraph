use directed_bijectional_connection_graph::DirectedBijectiveConnectionGraph;

fn main() {
    let graph = DirectedBijectiveConnectionGraph::new_hypercube(8);

    let s = 0b0011_0011;
    let d = 0b1010_1010;

    let path = graph.lemma2(s, d);

    println!("path from {:08b} to {:08b} in a hyper cube.", s, d);
    println!("{:?}", path);
}

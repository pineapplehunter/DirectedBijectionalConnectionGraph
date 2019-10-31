use directed_bijectional_connection_graph::DirectedBijectiveConnectionGraph;

fn main() {
    let graph = DirectedBijectiveConnectionGraph::new_hypercube(8);

    let src = 0b00110011;
    let dst = 0b10101010;

    let path = graph.lemma2(src, dst);

    println!("path from {:08b} to {:08b} in a hyper cube.", src, dst);
    println!("{:?}", path);
}

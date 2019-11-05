use directed_bijectional_connection_graph::hypercube::HypercubeGraph;
use directed_bijectional_connection_graph::Lemma1;

fn main() {
    let n = 8;
    let s = 0b0000_0000;

    let graph = HypercubeGraph::new(n);

    let path = graph.lemma1(n, s);

    println!("{:#?}", path);
}

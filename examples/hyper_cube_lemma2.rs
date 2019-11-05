use directed_bijectional_connection_graph::hypercube::HypercubeGraph;
use directed_bijectional_connection_graph::Lemma2;

fn main() {
    let n = 8;
    let s = 0b0011_0011;
    let d = 0b1010_1010;

    let graph = HypercubeGraph::new(n);

    let path = graph.lemma2(s, d);

    println!("{:?}", path);
}

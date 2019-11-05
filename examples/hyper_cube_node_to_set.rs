use directed_bijectional_connection_graph::hypercube::HypercubeGraph;
use directed_bijectional_connection_graph::NodeToSet;

fn main() {
    let n = 8;
    let s = 0b0101_0101;
    let mut d = vec![];

    for i in 0..8 {
        d.push(1 << i);
    }

    let graph = HypercubeGraph::new(n);

    let paths = graph.node_to_set(s, &d);

    println!("{:#?}", paths);
}

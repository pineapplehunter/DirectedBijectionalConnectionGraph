use directed_bijectional_connection_graph::Graph;

fn main() {
    let s = 0b0100;
    let d = 0b1101;
    let n = 4;

    let graph = Graph::new_hypercube(n);

    let path = graph.R(s, d);

    println!("{}", path);
}

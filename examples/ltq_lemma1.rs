use directed_bijectional_connection_graph::Lemma1;
use directed_bijectional_connection_graph::graphs::LocallyTwistedCube;

fn main() {
    let n = 8;
    let s = 0b0000_0001;

    let graph = LocallyTwistedCube::new(n);

    let path = graph.lemma1(n, s);

    println!("{:#?}", path);
}

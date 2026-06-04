use graphs::prelude::*;

const ACYCLIC: &str = "acyclic";
const CYCLIC: &str = "cyclic";

fn main() {
    let mut graph = SimpleGraph::<String, ()>::new();

    let (a, b, c) = (
        graph.add_node_value("a".to_owned()),
        graph.add_node_value("b".to_owned()),
        graph.add_node_value("c".to_owned()),
    );

    // a - b
    //  \ /
    //   c

    graph.extend_into_edges([(a, b), (b, c), (a, c)]).exhaust();

    let cyclic = graph.by_ref().apply(CyclicUndirected);

    let cycle = if cyclic { CYCLIC } else { ACYCLIC };

    println!("graph is {cycle}");
}

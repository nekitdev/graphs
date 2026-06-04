use graphs_dot::ast::DotGraph;

// A - B
//  \ /
//   C

const GRAPH: &str = r"
graph G {
    A -- B -- C -- A;
}
";

fn main() {
    let graph = DotGraph::parse_str(GRAPH).unwrap();

    println!("{graph:#?}");
}

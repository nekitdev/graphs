use graphs_dot::include_graph_str;

fn main() {
    let graph = include_graph_str!("graph G { a -- b -- c -- a }");

    println!("{graph:#?}")
}

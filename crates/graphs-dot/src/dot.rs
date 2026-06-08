use graphs_core::references::References;

pub struct Dot<G: References, N, E> {
    graph: G,
    get_node: N,
    get_edge: E,
}

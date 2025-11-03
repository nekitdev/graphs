use graphs::{
    core::{
        base::Base,
        find::{Find, OrNothing},
    },
    simple::SimpleGraph,
};

fn main() {
    let graph = SimpleGraph::<(), ()>::new();

    let output = graph.find_connecting(0, 1).or_nothing();
}

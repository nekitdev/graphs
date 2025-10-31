use graphs::{
    core::{base::Base, by::By, cycles::Cycles},
    simple::SimpleGraph,
};

fn main() {
    let graph = SimpleGraph::<(), ()>::new();

    let cycle = graph.by_ref().apply(Cycles);

    println!("{cycle:?}");
}

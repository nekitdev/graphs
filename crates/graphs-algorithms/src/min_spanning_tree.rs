use graphs_core::{algorithms::Algorithm, base::Base};
use graphs_union_find::vec::UnionFind;

pub struct MinSpanningForest<G> {
    graph: G,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MinSpanningTree;

impl<G: Base> Algorithm<G> for MinSpanningTree {
    type Output = MinSpanningForest<G>;

    fn perform(&mut self, graph: G) -> Self::Output {
        todo!()
    }
}

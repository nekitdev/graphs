use thiserror::Error;

use crate::{
    algorithm::Algorithm,
    base::Base,
    by::By,
    connections::Connection,
    identifiers::NodeIdentifiers,
    neighbors::Neighbors,
    recursive::{Back, dfs},
    visit::Visit,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("cycle caused by {connection}")]
pub struct Cycle<C: Connection> {
    pub connection: C,
}

impl<C: Connection> Cycle<C> {
    /// Constructs [`Self`].
    pub const fn new(connection: C) -> Self {
        Self { connection }
    }
}

pub type CycleIn<G> = Cycle<<G as Base>::Connection>;

pub type Output<G> = Option<CycleIn<G>>;

pub fn find<G: Visit + Neighbors + NodeIdentifiers>(graph: G) -> Output<G> {
    dfs(graph.by_ref(), graph.node_identifiers(), |event| {
        if let Back(connection) = event {
            Some(Cycle::new(connection))
        } else {
            None
        }
    })
}

pub struct Cycles;

impl<G: Visit + Neighbors + NodeIdentifiers> Algorithm<G> for Cycles {
    type Output = Output<G>;

    fn perform(&mut self, graph: G) -> Self::Output {
        find(graph)
    }
}

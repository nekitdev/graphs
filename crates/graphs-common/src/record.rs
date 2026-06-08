use graphs_core::{base::Base, id::NodeTypeId};

use crate::{id::DefaultNodeId, time::Time};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Times {
    pub discovered: Time,
    pub finished: Time,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Output {
    Undiscovered,
    Discovered(Time),
    Finished(Times),
}

pub use Output::{Discovered, Finished, Undiscovered};

impl Output {
    pub const fn is_undiscovered(self) -> bool {
        matches!(self, Self::Undiscovered)
    }

    pub const fn is_discovered(self) -> bool {
        matches!(self, Self::Discovered(_))
    }

    pub const fn is_finished(self) -> bool {
        matches!(self, Self::Finished(_))
    }
}

/// Represents recorders that keep track of discovery times.
pub trait Recorder<N: NodeTypeId = DefaultNodeId> {
    /// The associated type for possible [`record`] errors.
    ///
    /// [`record`]: Self::record
    type Error;

    /// Discovers the node with the given ID.
    fn discover(&mut self, node: N, time: Time);

    /// Finishes the node with the given ID.
    fn finish(&mut self, node: N, time: Time);

    /// Queries the node with the given ID.
    ///
    /// Returns the recorded time associated with the node, or [`None`] there is no record.
    fn query(&self, node: N) -> Output;
}

#[cfg(feature = "std")]
mod hash {
    use std::collections::HashMap;

    // impl<N: NodeTypeId> Recorder for HashMap<N, Output> {}
}

impl<N: NodeTypeId, R: Recorder<N>> Recorder<N> for &mut R {
    type Error = R::Error;

    fn discover(&mut self, node: N, time: Time) {
        (*self).discover(node, time)
    }

    fn finish(&mut self, node: N, time: Time) {
        (*self).finish(node, time)
    }

    fn query(&self, node: N) -> Output {
        (**self).query(node)
    }
}

/// Represents graphs which can be recorded.
pub trait Record: Base {
    /// The associated type for the recorder that can operate on this graph.
    type Recorder: Recorder<Self::NodeId>;

    /// Builds the recorder.
    fn build_recorder(&self) -> Self::Recorder;

    /// Resets the given recorder.
    fn reset_recorder(&self, recorder: &mut Self::Recorder);
}

impl<G: Record + ?Sized> Record for &G {
    type Recorder = G::Recorder;

    fn build_recorder(&self) -> Self::Recorder {
        (*self).build_recorder()
    }

    fn reset_recorder(&self, recorder: &mut Self::Recorder) {
        (*self).reset_recorder(recorder)
    }
}

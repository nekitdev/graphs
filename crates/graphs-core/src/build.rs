use core::error::Error;

use crate::{data::Data, internal::failed};

pub trait Build: Data {
    type NodeError: Error;
    type EdgeError: Error;

    fn try_add_node(&mut self, value: Self::NodeValue) -> Result<Self::NodeId, Self::NodeError>;

    fn try_add_edge(
        &mut self,
        source: Self::NodeId,
        target: Self::NodeId,
        value: Self::EdgeValue,
    ) -> Result<Self::EdgeId, Self::EdgeError>;

    fn add_node(&mut self, value: Self::NodeValue) -> Self::NodeId {
        self.try_add_node(value).expect(failed!(add_node))
    }

    fn add_edge(
        &mut self,
        source: Self::NodeId,
        target: Self::NodeId,
        value: Self::EdgeValue,
    ) -> Self::EdgeId {
        self.try_add_edge(source, target, value)
            .expect(failed!(add_edge))
    }
}

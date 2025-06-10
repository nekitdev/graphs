use core::error::Error;

use crate::{
    data::Data,
    id::{DefaultId, Id},
    internal::failed,
};

pub struct Output<T, I: Id = DefaultId> {
    pub index: I,
    pub previous: Option<T>,
}

pub trait Update: Data {
    type UpdateError: Error;

    fn try_update_edge(
        &mut self,
        source: Self::NodeId,
        target: Self::NodeId,
        value: Self::EdgeValue,
    ) -> Result<UpdateOutput<Self::EdgeValue, Self::EdgeId>, Self::UpdateError>
    where
        Self::EdgeValue: Sized;

    fn update_edge(
        &mut self,
        source: Self::NodeId,
        target: Self::NodeId,
        value: Self::EdgeValue,
    ) -> UpdateOutput<Self::EdgeValue, Self::EdgeId> {
        self.try_update_edge(source, target, value)
            .expect(failed!(update_edge))
    }
}

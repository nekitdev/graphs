use crate::{
    id::{EdgeTypeId, NodeTypeId},
    kinds::{self, Kind},
};

pub mod id;

pub trait Connection: EdgeTypeId {
    type Id: NodeTypeId;
    type Kind: Kind;
    type Inverse: Connection<Id = Self::Id, Kind = <Self::Kind as Kind>::Inverse>;

    fn connecting(one: Self::Id, two: Self::Id) -> Self;
}

pub trait Directed: Connection<Kind = kinds::Directed> {}
pub trait Undirected: Connection<Kind = kinds::Undirected> {}

impl<C: Connection<Kind = kinds::Directed>> Directed for C {}
impl<C: Connection<Kind = kinds::Undirected>> Undirected for C {}

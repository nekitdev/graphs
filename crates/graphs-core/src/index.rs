use trait_aliases::trait_aliases;

use crate::id::{EdgeType, Id, NodeType, NoneType};

pub const OF: &str = "can not convert index to id";
pub const INDEX: &str = "can not convert id to index";

pub trait Index: Id {
    fn try_of(index: usize) -> Option<Self>;
    fn try_index(self) -> Option<usize>;

    #[must_use]
    fn of(index: usize) -> Self {
        Self::try_of(index).expect(OF)
    }

    fn index(self) -> usize {
        self.try_index().expect(INDEX)
    }
}

trait_aliases! {
    #[trait_alias(I)]
    pub trait UntypedIndex = Index<Type = NoneType>;

    #[trait_alias(N)]
    pub trait NodeTypeIndex = Index<Type = NodeType>;

    #[trait_alias(E)]
    pub trait EdgeTypeIndex = Index<Type = EdgeType>;
}

macro_rules! impl_untyped_index {
    ($($int: ty),* $(,)?) => {
        $(
            impl $crate::index::Index for $int {
                fn try_of(index: usize) -> Option<Self> {
                    index.try_into().ok()
                }

                fn try_index(self) -> Option<usize> {
                    self.try_into().ok()
                }
            }
        )*
    };
}

impl_untyped_index!(u8, u16, u32, u64, u128, usize);

pub type DefaultUntypedIndex = usize;

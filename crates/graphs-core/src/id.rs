use core::{fmt, hash::Hash};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use trait_aliases::trait_aliases;

use crate::{markers::Private, sentinel::Sentinel};

mod sealed {
    pub trait Sealed {}
}

/// Represents [`Id`] types.
pub trait Type: sealed::Sealed {}

/// Represents *untyped* [`Id`] type.
pub struct NoneType {
    private: Private,
}

/// Represents *node* [`Id`] type.
pub struct NodeType {
    private: Private,
}

/// Represents *edge* [`Id`] type.
pub struct EdgeType {
    private: Private,
}

impl sealed::Sealed for NoneType {}
impl sealed::Sealed for NodeType {}
impl sealed::Sealed for EdgeType {}

impl Type for NoneType {}
impl Type for NodeType {}
impl Type for EdgeType {}

trait_aliases! {
    /// Represents core [`Id`] bounds.
    ///
    /// Implemented for all type satisfying said core bounds.
    #[trait_alias(I)]
    pub trait CoreId = Copy + Ord + Hash + Sentinel + fmt::Debug + fmt::Display;

    /// Represents [`serde`] bounds on [`Id`].
    ///
    /// Implemented for any types that are [`Serialize`] and [`Deserialize`] for any lifetime.
    #[cfg(feature = "serde")]
    #[trait_alias(I)]
    pub trait SerdeId = Serialize + for<'de> Deserialize<'de>;

    /// Represents [`Id`] bounds.
    #[cfg(not(feature = "serde"))]
    #[trait_alias(I)]
    pub trait BaseId = CoreId;

    /// Represents [`Id`] bounds.
    #[cfg(feature = "serde")]
    #[trait_alias(I)]
    pub trait BaseId = CoreId + SerdeId;
}

/// Represents identifiers in graphs.
pub trait Id: BaseId {
    /// The type of the identifier.
    type Type: Type;
}

trait_aliases! {
    /// Represents *untyped* identifiers.
    ///
    /// Implemented for any [`Id`] with [`Type`] set to [`NoneType`].
    ///
    /// [`Type`]: Id::Type
    #[trait_alias(I)]
    pub trait UntypedId = Id<Type = NoneType>;

    /// Represents *node* type identifiers.
    ///
    /// Implemented for any [`Id`] with [`Type`] set to [`NodeType`].
    ///
    /// [`Type`]: Id::Type
    #[trait_alias(N)]
    pub trait NodeTypeId = Id<Type = NodeType>;

    /// Represents *edge* type identifiers.
    ///
    /// Implemented for any [`Id`] with [`Type`] set to [`EdgeType`].
    ///
    /// [`Type`]: Id::Type
    #[trait_alias(E)]
    pub trait EdgeTypeId = Id<Type = EdgeType>;
}

macro_rules! impl_untyped_id {
    ($($int: ty),* $(,)?) => {
        $(
            impl $crate::id::Id for $int {
                type Type = $crate::id::NoneType;
            }
        )*
    };
}

impl_untyped_id!(u8, u16, u32, u64, u128, usize);

/// The default *untyped* [`Id`].
pub type DefaultUntypedId = usize;

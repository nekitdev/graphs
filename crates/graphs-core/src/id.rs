use core::{fmt, hash::Hash, marker::PhantomData};

mod sealed {
    pub trait Sealed {}
}

/// Represents [`Id`] types.
pub trait Type: sealed::Sealed {}

/// Represents *none* [`Id`] type.
pub struct NoneType {
    private: PhantomData<()>,
}

/// Represents *node* [`Id`] type.
pub struct NodeType {
    private: PhantomData<()>,
}

/// Represents *edge* [`Id`] type.
pub struct EdgeType {
    private: PhantomData<()>,
}

impl sealed::Sealed for EdgeType {}
impl sealed::Sealed for NodeType {}
impl sealed::Sealed for NoneType {}

impl Type for EdgeType {}
impl Type for NodeType {}
impl Type for NoneType {}

/// Represents identifiers in graphs.
pub trait Id: Copy + Ord + Hash + Default {
    /// The type of the identifier.
    type Type: Type;

    /// The limiting value of the identifier.
    ///
    /// This is used to represent non-existing nodes or edges.
    const LIMIT: Self;

    /// Checks if the identifier is the limit.
    ///
    /// The default implementation simply compares [`Self`] with [`LIMIT`].
    ///
    /// [`LIMIT`]: Self::LIMIT
    fn is_limit(self) -> bool {
        self == Self::LIMIT
    }
}

pub trait NoneTypeId: Id<Type = NoneType> {}
pub trait NodeTypeId: Id<Type = NodeType> {}
pub trait EdgeTypeId: Id<Type = EdgeType> {}

impl<I: Id<Type = NoneType>> NoneTypeId for I {}
impl<N: Id<Type = NodeType>> NodeTypeId for N {}
impl<E: Id<Type = EdgeType>> EdgeTypeId for E {}

macro_rules! impl_none_id_limit_via_max {
    ($($int: ty),+ $(,)?) => {
        $(
            impl $crate::id::Id for $int {
                type Type = $crate::id::NoneType;
                const LIMIT: Self = <$int>::MAX;
            }
        )+
    }
}

impl_none_id_limit_via_max!(u8, u16, u32, u64, u128, usize);

/// The default *untyped* [`Id`].
pub type DefaultNoneId = usize;

/// Represents *node* identifiers wrapping some *untyped* [`Id`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NodeId<I: NoneTypeId = DefaultNoneId> {
    inner: I,
}

/// Represents *edge* identifiers wrapping some *untyped* [`Id`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EdgeId<I: NoneTypeId = DefaultNoneId> {
    inner: I,
}

impl<I: NoneTypeId + fmt::Display> fmt::Display for NodeId<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<I: NoneTypeId + fmt::Display> fmt::Display for EdgeId<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<I: NoneTypeId> NodeId<I> {
    /// Constructs [`Self`].
    pub const fn new(inner: I) -> Self {
        Self { inner }
    }

    /// Returns the contained *untyped* identifier.
    pub const fn get(self) -> I {
        self.inner
    }
}

impl<I: NoneTypeId> EdgeId<I> {
    /// Constructs [`Self`].
    pub const fn new(inner: I) -> Self {
        Self { inner }
    }

    /// Returns the contained *untyped* identifier.
    pub const fn get(self) -> I {
        self.inner
    }
}

impl<I: NoneTypeId> Id for NodeId<I> {
    type Type = NodeType;

    const LIMIT: Self = Self::new(I::LIMIT);

    fn is_limit(self) -> bool {
        self.get().is_limit()
    }
}

impl<I: NoneTypeId> Id for EdgeId<I> {
    type Type = EdgeType;

    const LIMIT: Self = Self::new(I::LIMIT);

    fn is_limit(self) -> bool {
        self.get().is_limit()
    }
}

/// The default *node* [`Id`].
pub type DefaultNodeId = NodeId<DefaultNoneId>;

/// The default *edge* [`Id`].
pub type DefaultEdgeId = EdgeId<DefaultNoneId>;

/// Shorthand for calling [`new`] on [`NodeId`].
///
/// [`new`]: NodeId::new
pub const fn node_id<I: NoneTypeId>(inner: I) -> NodeId<I> {
    NodeId::new(inner)
}

/// Shorthand for calling [`new`] on [`EdgeId`].
///
/// [`new`]: EdgeId::new
pub const fn edge_id<I: NoneTypeId>(inner: I) -> EdgeId<I> {
    EdgeId::new(inner)
}

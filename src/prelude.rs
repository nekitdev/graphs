//! `graphs` prelude.

#[doc(inline)]
pub use crate::core::prelude::*;

#[doc(inline)]
#[cfg(feature = "common")]
pub use crate::common::prelude::*;

#[doc(inline)]
#[cfg(feature = "simple")]
pub use crate::simple::prelude::*;

#[doc(inline)]
#[cfg(feature = "stable")]
pub use crate::stable::prelude::*;

#[doc(inline)]
#[cfg(feature = "algorithms")]
pub use crate::algorithms::prelude::*;

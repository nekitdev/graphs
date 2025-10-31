use core::fmt;

use crate::markers::{Marker, Private, StaticStr};

mod sealed {
    pub trait Sealed {}
}

pub trait Loop: Marker + sealed::Sealed {
    const FORBID: bool;

    type Inverse: Loop<Inverse = Self>;
}

pub struct Forbid {
    private: Private,
}

pub struct Allow {
    private: Private,
}

pub const FORBID: &str = "forbid";
pub const ALLOW: &str = "allow";

impl Marker for Forbid {
    const NAME: StaticStr = stringify!(Forbid);

    fn display(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(FORBID)
    }
}

impl Marker for Allow {
    const NAME: StaticStr = stringify!(Allow);

    fn display(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(ALLOW)
    }
}

impl sealed::Sealed for Forbid {}
impl sealed::Sealed for Allow {}

impl Loop for Forbid {
    const FORBID: bool = true;

    type Inverse = Allow;
}

impl Loop for Allow {
    const FORBID: bool = false;

    type Inverse = Forbid;
}

pub type DefaultLoop = Forbid;

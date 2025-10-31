use core::fmt;

use crate::markers::{Marker, Private, StaticStr};

mod sealed {
    pub trait Sealed {}
}

pub trait Type: Marker + sealed::Sealed {
    const SINGLE: bool;

    type Inverse: Type<Inverse = Self>;
}

pub struct Single {
    private: Private,
}

pub struct Multiple {
    private: Private,
}

pub const SINGLE: &str = "single";
pub const MULTIPLE: &str = "multiple";

impl Marker for Single {
    const NAME: StaticStr = stringify!(Single);

    fn display(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(SINGLE)
    }
}

impl Marker for Multiple {
    const NAME: StaticStr = stringify!(Multiple);

    fn display(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(MULTIPLE)
    }
}

impl sealed::Sealed for Single {}
impl sealed::Sealed for Multiple {}

impl Type for Single {
    const SINGLE: bool = true;

    type Inverse = Multiple;
}

impl Type for Multiple {
    const SINGLE: bool = false;

    type Inverse = Single;
}

pub type DefaultType = Single;

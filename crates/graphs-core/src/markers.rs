use core::{fmt, marker::PhantomData};

pub type Private = PhantomData<()>;

pub type StaticStr = &'static str;

pub trait Marker {
    const NAME: StaticStr;

    fn display(formatter: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub trait MarkerOutput: Marker {
    fn output() -> Output<Self> {
        Output::new()
    }
}

impl<M: Marker + ?Sized> MarkerOutput for M {}

pub struct Output<M: Marker + ?Sized> {
    marker: PhantomData<M>,
}

impl<M: Marker + ?Sized> fmt::Debug for Output<M> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(M::NAME)
    }
}

impl<M: Marker + ?Sized> fmt::Display for Output<M> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        M::display(formatter)
    }
}

impl<M: Marker + ?Sized> Output<M> {
    pub(crate) const fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

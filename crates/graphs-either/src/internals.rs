use core::fmt;

#[inline(never)]
#[cold]
#[track_caller]
pub(crate) fn fail(message: &str, error: &dyn fmt::Debug) -> ! {
    panic!("{message}: {error:?}");
}

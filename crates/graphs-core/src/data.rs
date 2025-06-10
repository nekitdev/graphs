//! Various traits for handling data in graphs.

pub trait Ref {
    type Value;

    fn get(&self) -> &Self::Value;
}

pub trait Mut: Ref {
    fn get_mut(&mut self) -> &mut Self::Value;
}

pub trait Owned: Mut {
    fn take(self) -> Self::Value;
}

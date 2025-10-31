use crate::base::Directed;

pub trait Reverse: Directed {
    fn reverse(&mut self);
}

impl<G: Reverse + ?Sized> Reverse for &mut G {
    fn reverse(&mut self) {
        (*self).reverse();
    }
}

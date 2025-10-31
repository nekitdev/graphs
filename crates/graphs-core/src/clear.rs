pub trait Clear {
    fn clear(&mut self);
}

pub trait ClearEdges: Clear {
    fn clear_edges(&mut self);
}

impl<G: Clear + ?Sized> Clear for &mut G {
    fn clear(&mut self) {
        (*self).clear();
    }
}

impl<G: ClearEdges + ?Sized> ClearEdges for &mut G {
    fn clear_edges(&mut self) {
        (*self).clear_edges();
    }
}

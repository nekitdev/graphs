pub trait Exhaust: Iterator {
    fn exhaust(&mut self) {
        self.for_each(drop);
    }
}

impl<I: Iterator + ?Sized> Exhaust for I {}

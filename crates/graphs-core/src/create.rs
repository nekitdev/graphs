use crate::{base::Base, capacity::Capacities};

pub trait Create: Base {
    fn empty() -> Self;

    fn with_capacity(capacities: Capacities) -> Self;
}

use core::fmt;

use crate::data::Data;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Node<T> {
    pub value: T,
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value().fmt(formatter)
    }
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Node<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> &T {
        &self.value
    }

    pub const fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn get(self) -> T {
        self.value
    }
}

pub type NodeIn<G> = Node<<G as Data>::NodeValue>;

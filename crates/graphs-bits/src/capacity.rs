pub enum Capacity {
    Bits(usize),
    Blocks(usize),
}

pub use Capacity::{Bits, Blocks};

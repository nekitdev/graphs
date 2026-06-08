#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

pub mod ast;
pub mod owned;
pub mod parser;
pub mod prelude;

cfg_select! {
    feature = "std" => {
        pub mod canonical;
        pub mod filter;
    }
    _ => {}
}

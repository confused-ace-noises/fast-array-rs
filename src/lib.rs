#![cfg_attr(feature = "nightly", feature(portable_simd, step_trait))]

#![allow(soft_unstable)]


pub mod fast_array;
pub mod fast_iterator;
pub mod macros;
pub mod prelude;

pub use fast_array::fast_array::FastArray;
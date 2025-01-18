#![cfg_attr(feature = "nightly", feature(step_trait))]
#![cfg_attr(all(feature = "nightly", feature = "simd"), feature(portable_simd))]

#![allow(soft_unstable)]


pub mod fast_array;
pub mod fast_iterator;
pub mod macros;
pub mod prelude;

pub use fast_array::fast_array::FastArray;
pub use fast_iterator::fast_iterator::FastIterator;
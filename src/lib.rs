#![feature(step_trait)]
#![feature(portable_simd)]
#![feature(test)]

#![allow(soft_unstable)]


pub mod fast_array;
pub mod fast_iterator;
pub mod macros;

pub use fast_array::fast_array::FastArray;


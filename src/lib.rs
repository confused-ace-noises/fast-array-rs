//! # FAST-COLLECTIONS
//! `fast-collections` is a library that promises high performance collections with low-level manipulation.
//! 
//! by philosophy, no collection in this library is resizable.
//! 
//! 

#![cfg_attr(feature = "nightly", feature(step_trait))]
#![cfg_attr(all(feature = "nightly", feature = "simd"), feature(portable_simd))]
// #![feature(portable_simd)]
// #![feature(step_trait)]
#![allow(soft_unstable)]

pub mod fast_array;
pub mod fast_iterator;
pub mod macros;
pub mod prelude;
pub mod fast_matrix;

// use std::{arch::x86_64::{_mm_prefetch, _MM_HINT_T0}, simd::{Simd, SimdElement}};

pub use fast_array::fast_array::FastArray;
pub use fast_iterator::fast_iterator::FastIterator;
pub use fast_matrix::fast_matrix::FastMatrix;
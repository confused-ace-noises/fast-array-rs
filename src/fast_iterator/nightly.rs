#![cfg(feature = "nightly")]
use std::{iter::Step, ops::{Range, RangeInclusive}};

use crate::FastArray;

impl<T: Step+Clone> From<Range<T>> for FastArray<T> {
    #[inline(always)]
    fn from(mut value: Range<T>) -> Self {
        let len = value.clone().count();
        let func = |_| value.next().unwrap();
        FastArray::new_func(len, func)
    }
}

impl<T: Step+Clone> From<RangeInclusive<T>> for FastArray<T> {
    #[inline(always)]
    fn from(mut value: RangeInclusive<T>) -> Self {
        let len = value.clone().count();
        let func = |_| value.next().unwrap();
        FastArray::new_func(len, func)
    }
}
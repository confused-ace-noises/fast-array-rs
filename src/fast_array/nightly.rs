#![cfg(feature = "nightly")]
use std::{arch::x86_64::{_mm_prefetch, _MM_HINT_T0}, cmp::Ordering, ops::AddAssign, simd::{f32x4, f32x8, Simd, SimdElement}};

use std::iter::Step;
use crate::FastArray;

impl<T: Step + std::fmt::Debug + Copy> FastArray<T> {
    /// # Info
    /// creates a new [`FastArray`] with a start and end, as you'd do with a range.
    /// the range equivalent to this is (start..end).
    ///
    /// might bring performance gains or hits compared to doing it with ranges based on the situation.
    /// 
    /// ## Example
    /// ```
    /// let fast_arr = (0..10).into().as_fast_array(); // use into() for best performance
    /// let fast_arr_range = FastArray::new_range(0, 10);
    /// 
    /// assert_eq!(fast_arr, fast_arr_range);
    /// ```
    pub fn new_range(start: T, end: T) -> FastArray<T> {
        assert_ne!(start, end, "Start and end must not be equal!");

        let (_ , Some(len)) = T::steps_between(&start, &end) else {
            panic!("only known steps in-between are allowed")
        };

        let mut empty_arr = unsafe { FastArray::<T>::new_empty(len) };

        let mut value = start;
        let mut index = 0;
        while &value < &end {
            empty_arr[index] = value;
            value = T::forward(value, 1);
            index+=1;
        }

        empty_arr
    }

    /// ## Info
    /// same functionality as [`FastArray::new_range`], just skips the `start != end` check for performance reasons.
    /// if `start == end`, this function ends up being undefined behavior.
    pub unsafe fn new_range_unchecked(start: T, end: T) -> FastArray<T> {
        // assert_ne!(start, end, "Start and end must not be equal!");

        let (_ , Some(len)) = T::steps_between(&start, &end) else {
            panic!("only known steps in-between are allowed")
        };

        let mut empty_arr = unsafe { FastArray::<T>::new_empty(len) };

        let mut value = start;
        let mut index = 0;
        while &value < &end {
            empty_arr[index] = value;
            value = T::forward(value, 1);
            index+=1;
        }

        empty_arr
    }
}

impl<T> FastArray<T>
where
    T: SimdElement + Copy + std::ops::Add<Output = T> + AddAssign,  // Ensure T supports addition
    Simd<T, 4>: std::ops::Add<Output = Simd<T, 4>>,     // Ensure SIMD type supports addition
{
    #[inline(always)]
    pub fn simd_add(&mut self, other: T) {
        let len = self.size;
        type WideSimd<T> = Simd<T, 4>;
        let lanes = WideSimd::<T>::LEN;
        let mut i = 0;

        // 🔥 New: Align pointer before SIMD processing
        while i < len && (unsafe { self.pointer.add(i) } as usize) % std::mem::align_of::<WideSimd<T>>() != 0 {
            unsafe {
                *self.pointer.add(i) += other;
            }
            i += 1;
        }

        // 🔥 SIMD Processing
        while i + lanes <= len {
            unsafe {
                _mm_prefetch(self.pointer.add(i + 64) as *const i8, _MM_HINT_T0);
                let av = *(self.pointer.add(i) as *const WideSimd<T>);
                            let bv = WideSimd::splat(other);
                            let result = av + bv;
                            *(self.pointer.add(i) as *mut WideSimd<T>) = result; // SIMD store
            }
            i += lanes;
        }

        // 🔥 Scalar cleanup (if remainder exists)
        while i < len {
            unsafe {
                *self.pointer.add(i) += other;
            }
            i += 1;
        }
    }

    // #[inline(always)]
    // pub fn simd_add(&mut self, other: T) {
    //     assert_eq!((self.pointer as usize) % 32, 0, "Pointer self is not 32-byte aligned!");
    //     // assert_eq!((other.pointer as usize) % 32, 0, "Pointer other is not 32-byte aligned!");
    //     // assert_eq!(self.size, other.size, "Arrays must have the same size!");
    //     let len = self.size;

    //     type WideSimd<T> = Simd<T, 8>; // Use a fixed SIMD width
    //     let lanes = WideSimd::<T>::LEN;
    //     let mut i = 0;

    //     while i + lanes <= len {
    //         unsafe {
    //             // let av = WideSimd::from_slice(std::slice::from_raw_parts(self.pointer.add(i), lanes));
    //             let av = *(self.pointer.add(i) as *const WideSimd<T>);
    //             let bv = WideSimd::splat(other);
    //             let result = av + bv; // Now this compiles because T supports addition
    //             // result.copy_to_slice(std::slice::from_raw_parts_mut(self.pointer.add(i), lanes));
    //             *(self.pointer.add(i) as *mut WideSimd<T>) = result; // Aligned SIMD store
    //             // result.store_select(slice, enable);
    //         }
    //         i += lanes;
    //     }

    //     // Handle remaining scalar elements
    //     while i < len {
    //         unsafe {
    //             *self.pointer.add(i) = *self.pointer.add(i) + other;
    //         }
    //         i += 1;
    //     }
    // }
    
    pub fn simd_add_array(&mut self, other: &FastArray<T>) {
        // assert_eq!((self.pointer as usize) % 32, 0, "Pointer self is not 32-byte aligned!");
        // assert_eq!((other.pointer as usize) % 32, 0, "Pointer other is not 32-byte aligned!");
        assert_eq!(self.size, other.size, "Arrays must have the same size!");
        let len = self.size;

        type WideSimd<T> = Simd<T, 4>; // Use a fixed SIMD width
        let lanes = WideSimd::<T>::LEN;
        let mut i = 0;

        while i + lanes <= len {
            unsafe {
                let av = WideSimd::from_slice(std::slice::from_raw_parts(self.pointer.add(i), lanes));
                let bv = WideSimd::from_slice(std::slice::from_raw_parts(other.pointer.add(i), lanes));
                let result = av + bv; // Now this compiles because T supports addition
                // result.copy_to_slice(std::slice::from_raw_parts_mut(self.pointer.add(i), lanes));
                *(self.pointer.add(i) as *mut WideSimd<T>) = result; // Aligned SIMD store
            }
            i += lanes;
        }

        // Handle remaining scalar elements
        while i < len {
            unsafe {
                *self.pointer.add(i) = *self.pointer.add(i) + *other.pointer.add(i);
            }
            i += 1;
        }
    }
}

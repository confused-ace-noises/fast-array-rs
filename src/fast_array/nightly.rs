#![cfg(feature = "nightly")]
// use std::iter::Step;
use std::cmp::Ordering;

use crate::FastArray;
use std::iter::Step;

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

        let (_, Some(len)) = T::steps_between(&start, &end) else {
            panic!("only known steps in-between are allowed")
        };

        let mut empty_arr = unsafe { FastArray::<T>::new_empty(len) };

        let mut value = start;
        let mut index = 0;
        while &value < &end {
            empty_arr[index] = value;
            value = T::forward(value, 1);
            index += 1;
        }

        empty_arr
    }

    /// ## Info
    /// same functionality as [`FastArray::new_range`], just skips the `start != end` check for performance reasons.
    /// if `start == end`, this function ends up being undefined behavior.
    pub unsafe fn new_range_unchecked(start: T, end: T) -> FastArray<T> {
        // assert_ne!(start, end, "Start and end must not be equal!");

        let (_, Some(len)) = T::steps_between(&start, &end) else {
            panic!("only known steps in-between are allowed")
        };

        let mut empty_arr = unsafe { FastArray::<T>::new_empty(len) };

        let mut value = start;
        let mut index = 0;
        while &value < &end {
            empty_arr[index] = value;
            value = T::forward(value, 1);
            index += 1;
        }

        empty_arr
    }
}

#[cfg(all(feature = "simd", feature = "nightly"))]
pub mod simd {
    use crate::FastArray;
    use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
    use std::ops::{Add, Mul};
    use std::simd::{f32x4, f32x8, LaneCount, Simd, SimdElement, SupportedLaneCount};
    // use crate::make_simd;
    // use std::concat_idents;

    impl<T: Copy + Default + Add<Output = T> + Mul<Output = T> + std::iter::Sum + SimdElement>
        FastArray<T>
    {
        const PREFETCH_DISTANCE: usize = 128;

        // 🚀 SIMD Addition
        pub fn simd_add_2_lanes(&mut self, other: T)
        where
            Simd<T, 2>: Add<Output = Simd<T, 2>>,
        {
            self.simd_add_generic::<2>(other);
        }
        pub fn simd_add_4_lanes(&mut self, other: T)
        where
            Simd<T, 4>: Add<Output = Simd<T, 4>>,
        {
            self.simd_add_generic::<4>(other);
        }
        pub fn simd_add_8_lanes(&mut self, other: T)
        where
            Simd<T, 8>: Add<Output = Simd<T, 8>>,
        {
            self.simd_add_generic::<8>(other);
        }
        pub fn simd_add_16_lanes(&mut self, other: T)
        where
            Simd<T, 16>: Add<Output = Simd<T, 16>>,
        {
            self.simd_add_generic::<16>(other);
        }
        pub fn simd_add_32_lanes(&mut self, other: T)
        where
            Simd<T, 32>: Add<Output = Simd<T, 32>>,
        {
            self.simd_add_generic::<32>(other);
        }
        pub fn simd_add_64_lanes(&mut self, other: T)
        where
            Simd<T, 64>: Add<Output = Simd<T, 64>>,
        {
            self.simd_add_generic::<64>(other);
        }

        // 🚀 SIMD Multiplication
        pub fn simd_mul_2_lanes(&mut self, other: T)
        where
            Simd<T, 2>: Mul<Output = Simd<T, 2>>,
        {
            self.simd_mul_generic::<2>(other);
        }
        pub fn simd_mul_4_lanes(&mut self, other: T)
        where
            Simd<T, 4>: Mul<Output = Simd<T, 4>>,
        {
            self.simd_mul_generic::<4>(other);
        }
        pub fn simd_mul_8_lanes(&mut self, other: T)
        where
            Simd<T, 8>: Mul<Output = Simd<T, 8>>,
        {
            self.simd_mul_generic::<8>(other);
        }
        pub fn simd_mul_16_lanes(&mut self, other: T)
        where
            Simd<T, 16>: Mul<Output = Simd<T, 16>>,
        {
            self.simd_mul_generic::<16>(other);
        }
        pub fn simd_mul_32_lanes(&mut self, other: T)
        where
            Simd<T, 32>: Mul<Output = Simd<T, 32>>,
        {
            self.simd_mul_generic::<32>(other);
        }
        pub fn simd_mul_64_lanes(&mut self, other: T)
        where
            Simd<T, 64>: Mul<Output = Simd<T, 64>>,
        {
            self.simd_mul_generic::<64>(other);
        }

        // 🚀 SIMD Dot Product
        pub fn simd_dot_2_lanes(&self, other: &FastArray<T>) -> T
        where
            Simd<T, 2>: Add<Output = Simd<T, 2>> + Mul<Output = Simd<T, 2>>,
        {
            self.simd_dot_generic::<2>(other)
        }
        pub fn simd_dot_4_lanes(&self, other: &FastArray<T>) -> T
        where
            Simd<T, 4>: Add<Output = Simd<T, 4>> + Mul<Output = Simd<T, 4>>,
        {
            self.simd_dot_generic::<4>(other)
        }
        pub fn simd_dot_8_lanes(&self, other: &FastArray<T>) -> T
        where
            Simd<T, 8>: Add<Output = Simd<T, 8>> + Mul<Output = Simd<T, 8>>,
        {
            self.simd_dot_generic::<8>(other)
        }
        pub fn simd_dot_16_lanes(&self, other: &FastArray<T>) -> T
        where
            Simd<T, 16>: Add<Output = Simd<T, 16>> + Mul<Output = Simd<T, 16>>,
        {
            self.simd_dot_generic::<16>(other)
        }
        pub fn simd_dot_32_lanes(&self, other: &FastArray<T>) -> T
        where
            Simd<T, 32>: Add<Output = Simd<T, 32>> + Mul<Output = Simd<T, 32>>,
        {
            self.simd_dot_generic::<32>(other)
        }
        pub fn simd_dot_64_lanes(&self, other: &FastArray<T>) -> T
        where
            Simd<T, 64>: Add<Output = Simd<T, 64>> + Mul<Output = Simd<T, 64>>,
        {
            self.simd_dot_generic::<64>(other)
        }

        fn simd_add_generic<const N: usize>(&mut self, other: T)
        where
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: Add<Output = Simd<T, N>>, // ✅ Explicit per-lane Add support
        {
            // assert!(self.pointer as usize % 32 == 0, "Memory not properly aligned!");

            // type WideSimd<T, const N: usize> = Simd<T, N>;
            let lanes = Simd::<T, N>::LEN;
            let mut i = 0;

            while i + lanes <= self.size {
                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    _mm_prefetch(
                        self.pointer.add(i + Self::PREFETCH_DISTANCE).cast(),
                        _MM_HINT_T0,
                    );

                    let av = *(self.pointer.add(i) as *const Simd<T, N>);
                    let bv = Simd::splat(other);
                    *(self.pointer.add(i) as *mut Simd<T, N>) = av + bv;
                }
                i += lanes;
            }

            while i < self.size {
                let x = unsafe { self.pointer.add(i) };
                unsafe {
                    *x = *x + other;
                }
                i += 1;
            }
        }

        fn simd_mul_generic<const N: usize>(&mut self, other: T)
        where
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: Mul<Output = Simd<T, N>>, // ✅ Explicit per-lane Mul support
        {
            // type WideSimd<T, const N: usize> = Simd<T, N>;
            let lanes = Simd::<T, N>::LEN;
            let mut i = 0;

            while i + lanes <= self.size {
                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    _mm_prefetch(
                        self.pointer.add(i + Self::PREFETCH_DISTANCE).cast(),
                        _MM_HINT_T0,
                    );

                    let av = *(self.pointer.add(i) as *const Simd<T, N>);
                    let bv = Simd::splat(other);
                    *(self.pointer.add(i) as *mut Simd<T, N>) = av * bv;
                }
                i += lanes;
            }

            while i < self.size {
                let x = unsafe { self.pointer.add(i) };
                unsafe {
                    *x = *x * other;
                }
                i += 1;
            }
        }

        fn simd_dot_generic<const N: usize>(&self, other: &FastArray<T>) -> T
        where
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: Mul<Output = Simd<T, N>> + Add<Output = Simd<T, N>>, // ✅ Mul & Add for dot product
        {
            // type WideSimd<T, const N: usize> = Simd<T, N>;
            let lanes = Simd::<T, N>::LEN;
            let mut i = 0;
            let mut sum = Simd::<T, N>::splat(T::default());

            while i + lanes <= self.size {
                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    _mm_prefetch(
                        self.pointer.add(i + Self::PREFETCH_DISTANCE).cast(),
                        _MM_HINT_T0,
                    );
                    #[cfg(target_arch = "x86_64")]
                    _mm_prefetch(
                        other.pointer.add(i + Self::PREFETCH_DISTANCE).cast(),
                        _MM_HINT_T0,
                    );

                    let av = *(self.pointer.add(i) as *const Simd<T, N>);
                    let bv = *(other.pointer.add(i) as *const Simd<T, N>);
                    sum += av * bv;
                }
                i += lanes;
            }

            let mut scalar_sum = sum.to_array().into_iter().sum();
            
            while i < self.size {
                unsafe {
                    scalar_sum = scalar_sum + (*self.pointer.add(i) * *other.pointer.add(i));
                }
                i += 1;
            }

            scalar_sum
        }
    }
}
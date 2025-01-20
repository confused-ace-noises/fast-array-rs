// // #[macro_export]
// pub(crate) mod private {
//     macro_rules! count {
//         ($($val:expr),+) => {
//             <[()]>::len(&[$(count!(@substitute $val)),+])
//         };
//         (@substitute $_val:expr) => {
//             ()
//         };
//     }

//     pub(crate) use count;
// }

pub fn useless_fn<T>(_: &T) -> () {}

#[macro_export]
macro_rules! fast_arr {
    [$type:ty: $($val:expr),+$(,)?] => {
        {
            use $crate::fast_array::fast_array::FastArray;
            use $crate::macros::useless_fn;

            let mut fast_arr;

            unsafe {
                // fast_arr = FastArray::<$type>::new_empty(count!($($val),+));
                fast_arr = FastArray::<$type>::new_empty(<[()]>::len(&[
                    $(
                        {let _ = &$val; ()},
                    )+
                ]));

            }

            let mut index = 0;

            $(
                fast_arr[index] = $val;
                index += 1;
            )+

            fast_arr
        }
    };

    [$type:ty: $value:expr; $num:expr] => {
        {
            use $crate::fast_array::fast_array::FastArray;

            let closure = || {
                $value
            };
            let fast_arr = FastArray::<$type>::new_func($num, closure);
            fast_arr
        }
    };

    [$($val:expr),+$(,)?] => {
        {
            use $crate::fast_array::fast_array::FastArray;
            use $crate::macros::useless_fn;

            let mut fast_arr;

            unsafe {
                // fast_arr = FastArray::new_empty(count!($($val),+));
                fast_arr = FastArray::new_empty(<[()]>::len(&[$({let _ = &$val; ()}),+]));
            }

            let mut index = 0;

            $(
                fast_arr[index] = $val;
                index += 1;
            )+

            fast_arr
        }
    };

    [$value:expr; $num:expr] => {
        {
            use $crate::fast_array::fast_array::FastArray;

            let closure = || {
                $value
            };
            let fast_arr = FastArray::new_func($num, closure);
            fast_arr
        }
    }
}

#[test]
fn test() {
    let fast_arr = fast_arr!(1, 2, 3, 4, 5);
    drop(fast_arr);
}

#[macro_export]
macro_rules! make_simd {
    ($($n_lanes:expr, $name_1:ident, $name_2:ident; ),+ $(,)?) => {
        $(
            impl<T> FastArray<T>
    where
        T: SimdElement + Copy + std::ops::Add<Output = T>, // Ensure T supports addition
        Simd<T, 4>: std::ops::Add<Output = Simd<T, 4>>, // Ensure SIMD type supports addition
    {
        #[inline(always)]
        pub fn simd_add(&mut self, other: T) {
            let len = self.size;
            type WideSimd<T> = Simd<T, 4>;
            let lanes = WideSimd::<T>::LEN;
            let mut i = 0;

            // New: Align pointer before SIMD processing
            while i < len
                && (unsafe { self.pointer.add(i) } as usize) % std::mem::align_of::<WideSimd<T>>()
                    != 0
            {
                unsafe {
                    *self.pointer.add(i) + *self.pointer.add(i) + other;
                }
                i += 1;
            }

            // SIMD Processing
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

            // Scalar cleanup (if remainder exists)
            while i < len {
                unsafe {
                    *self.pointer.add(i) = *self.pointer.add(i) + other;
                }
                i += 1;
            }
        }

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
                    let av = WideSimd::from_slice(std::slice::from_raw_parts(
                        self.pointer.add(i),
                        lanes,
                    ));
                    let bv = WideSimd::from_slice(std::slice::from_raw_parts(
                        other.pointer.add(i),
                        lanes,
                    ));
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
)+
    }
}

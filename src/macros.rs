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
    let fast_arr = fast_arr!(1,2,3,4,5);
    drop(fast_arr);
}
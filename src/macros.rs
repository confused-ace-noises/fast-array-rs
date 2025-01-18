#[macro_export]
macro_rules! count {
    ($($val:expr),+) => {
        <[()]>::len(&[$(count!(@substitute $val)),+])
    };
    (@substitute $_val:expr) => {
        ()
    };
}

#[macro_export]
macro_rules! fast_arr {
    [$type:ty: $($val:expr),+$(,)?] => {
        {
            use $crate::count;
            use $crate::fast_array::fast_array::FastArray;

            let mut fast_arr;

            unsafe {
                fast_arr = FastArray::<$type>::new_empty(count!($($val),+));
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
            use $crate::count;
            use $crate::fast_array::fast_array::FastArray;

            let mut fast_arr;

            unsafe {
                fast_arr = FastArray::new_empty(count!($($val),+));
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
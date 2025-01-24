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

pub fn useless_fn<T>(_: &T) -> () {
    use crate::fast_matrix;
    let x = fast_matrix!([1, 2, 3], [1, 2, 3]);
    drop(x);
}


#[macro_export]
/// ## Info
/// Create new [`FastArray`]s in a convenient way.
/// 
/// ## Examples
/// possible syntaxes:
/// ```
/// use fast_array::fast_arr;
/// 
/// let fast_arr1 = fast_arr!(1,2,3,4,5,6); // [1, 2, 3, 4, 5, 6]
/// 
/// let fast_arr2 = fast_arr!(3; 6) // [3, 3, 3, 3, 3, 3]
/// 
/// // repeating pattern syntax
/// let fast_arr3 = fast_arr!([1,2]; 3) // [1, 2, 1, 2, 1, 2]
/// ```
macro_rules! fast_arr {
    
    [$($val:expr),+$(,)?] => {
        {
            use $crate::fast_array::fast_array::FastArray;
            
            let mut fast_arr;
            
            unsafe {
                // fast_arr = FastArray::new_empty(count!($($val),+));
                fast_arr = FastArray::new_empty_unchecked(<[()]>::len(&[$({let _ = &$val; ()}),+]));
            }
            
            let mut index = 0;

            $(
                fast_arr[index] = $val;
                index += 1;
            )+

            fast_arr
        }
    };
    
    ([$($element:expr),+]; $reps:expr) => {
        {
            use $crate::FastArray;

            let n_elements: usize = $reps * [ $( {let _ = &$element; ()} ),+ ].len();

            let mut fast_arr = unsafe{ FastArray::new_empty_unchecked(n_elements) };

            let mut index: usize = 0;
            
            for _ in 0..$reps {
                $(
                    fast_arr[index] = $element;
                    index+=1;
                )+
            }
            
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
    };


    // [$type:ty: $($val:expr),+$(,)?] => {
    //     {
    //         use $crate::fast_array::fast_array::FastArray;
    //         use $crate::macros::useless_fn;
    
    //         let mut fast_arr;
    
    //         unsafe {
    //             // fast_arr = FastArray::<$type>::new_empty(count!($($val),+));
    //             fast_arr = FastArray::<$type>::new_empty(<[()]>::len(&[
    //                 $(
    //                     {let _ = &$val; ()},
    //                 )+
    //             ]));
    
    //         }
    
    //         let mut index = 0;
    
    //         $(
    //             fast_arr[index] = $val;
    //             index += 1;
    //         )+
    
    //         fast_arr
    //     }
    // };
    
    // [$type:ty: $value:expr; $num:expr] => {
    //     {
    //         use $crate::fast_array::fast_array::FastArray;
    
    //         let closure = || {
    //             $value
    //         };
    //         let fast_arr = FastArray::<$type>::new_func($num, closure);
    //         fast_arr
    //     }
    // };
}


#[macro_export]
/// ## Info
/// A convenient macro to make new [`FastMatrix`] instances, 
/// 
/// ## Examples
/// possible syntaxes:
/// ```
/// use fast_array::fast_matrix;
/// 
/// // [
/// //     [1, 2, 3],
/// //     [4, 5, 6],
/// //     [7, 8, 9]
/// // ]
/// let fast_matrix1 = fast_matrix!([1, 2, 3], [4, 5, 6], [7, 8, 9]);
/// 
/// // [
/// //    [1, 2, 3],
/// //    [1, 2, 3],
/// //    [1, 2, 3],
/// // ]
/// let fast_matrix2 = fast_matrix!([1, 2, 3]; 3);
/// 
/// // [
/// //    [4, 4, 4],
/// //    [5, 5, 5],
/// //    [6, 6, 6],
/// // ]
/// let fast_matrix3 = fast_matrix!([4; 3], [5; 3], [6; 3]);
/// 
/// // [
/// //   [1, 2, 1, 2, 1, 2],
/// //   [3, 3, 3, 3, 3, 3],
/// //   [4, 5, 6, 4, 5, 6],
/// // ]
/// let fast_matrix4 = fast_matrix!([1,2; 3], [3; 6], [4, 5, 6; 2]);
/// ```
macro_rules! fast_matrix {
    ($( [$( $element:expr ),+ $(,)?; $reps:expr ] ),+ $(,)?) => {
        {
            use $crate::FastMatrix;


            #[allow(non_snake_case)]
            let ROWS: usize = [ $( [$( {let _ = &$element; ()} ),+].as_slice()),+ ].len();

            #[allow(non_snake_case)]
            let COLUMNS: usize = [ $( $reps * [ $( {let _ = &$element; ()} ),+].len() ),+ ][0];


            // -------------- START ASSERT ----------------
            assert!( $( ( $reps * [$( {let _ = $element; ()} ),+].len() ) == COLUMNS )&&+,
                "the number of columns must be the same for each row! If you used a repeating syntax, all rows must have the same lenght of repeating pattern: 'fast_matrix!([1,2; 3], [3,4,5; 2], [6; 6])' is okay because each row has the same lenght; 'fast_matrix!([1,2; 3], [3,4,5; 2], [6; 5])' isn't because the last row doesn't have the same lenght as the first two."
            );
            // --------------- END ASSERT -----------------



            let mut fast_matrix = unsafe { FastMatrix::new_empty_unchecked(ROWS, COLUMNS) };


            let mut _row_index = 0;
            let mut _col_index = 0;


            $(
                let reps: usize = $reps;
                for _ in 0..reps {
                    $(
                        fast_matrix[(_row_index, _col_index)] = $element;
                        _col_index+=1;
                    )+
                }
                _col_index = 0;
                _row_index += 1;
            )+


            fast_matrix
        }
    };


    ([ $($element:expr),+ $(,)?]; $num:expr) => {
        {
            use $crate::fast_matrix::fast_matrix::FastMatrix;
            #[allow(non_snake_case)]
            let ROWS: usize = $num;
            #[allow(non_snake_case)]
            let COLUMNS: usize = [$({let _ = &$element; ()}),+].len();

            // assert!($( [$($element),+].len() == COLUMNS )&&+, "FastMatrix cant have different sized ROWS!");

            let mut fast_matrix = unsafe{ FastMatrix::new_empty_unchecked(ROWS, COLUMNS) };
            // let mut fast_matrix = unsafe{ FastMatrix::new_empty(6, 3) };

            let mut index_column = 0;

            for index_row in 0..$num {
                $(
                    fast_matrix[(index_row, index_column)] = $element;
                    index_column+=1;
                )+
                index_column = 0;
            }

            fast_matrix
        }
    };


    ($([ $($element:expr),+ $(,)?] ),+ $(,)?) => {
        {
            use $crate::fast_matrix::fast_matrix::FastMatrix;
            #[allow(non_snake_case)]
            let ROWS: usize = [ $( { let _ = [ $( {let _ = &$element; ()} ),+ ]; () } ),+ ].len();
            #[allow(non_snake_case)]
            let COLUMNS: usize = [ $( [$({let _ = &$element; ()}),+].len() ),+ ][0];

            assert!($( [$({let _ = &$element; ()}),+].len() == COLUMNS )&&+, "FastMatrix cant have different sized ROWS!");

            let mut fast_matrix = unsafe{ FastMatrix::new_empty_unchecked(ROWS, COLUMNS) };
            // let mut fast_matrix = unsafe{ FastMatrix::new_empty(6, 3) };

            let mut _index_row = 0;
            let mut _index_column = 0;

            $(
                $(
                    fast_matrix[(_index_row, _index_column)] = $element;
                    _index_column+=1;

                )+

                _index_column = 0;
                _index_row += 1;

            )+

            fast_matrix
        }
    };

    ($element:expr; $reps_rows:expr; $reps_cols:expr) => {
        {
            use $crate::FastMatrix;
            FastMatrix::new($reps_rows, $reps_cols, $element)
        }
    };
}

#[test]
fn test() {
    let fast_matrix: crate::FastMatrix<usize> = fast_matrix!([1,2; 3], [3; 6], [4, 5, 6; 2]);
    println!("{}", fast_matrix)
}

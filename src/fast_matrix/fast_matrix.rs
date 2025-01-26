use std::{alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout}, ops::{Index, IndexMut}, ptr};

use crate::{prelude::FastIterator, FastArray};

#[derive(Debug, Clone)]
#[repr(align(32))]
/// ## Info
/// A matrix, aka a 2d array with same the same width for each row.
/// 
/// [`FastMatrix`] stands up to its name, being *very* fast, because it's completely built on pointer arithmetic and stored in memory as a continuous array, but still behaving like a 2d one.
///
/// As everything else in this library, [`FastMatrix`] is not resizable.
/// 
/// ## Example
/// ```
/// use fast_arr::FastMatrix;
/// use fast_arr::fast_matrix;
/// 
/// // [
/// //     [1,2,3],
/// //     [4,5,6],
/// //     [7,8,9]
/// // ]
/// let fast_matrix = fast_matrix!([1,2,3], [4,5,6], [7,8,9])
/// 
/// let element = fast_matrix[(1, 2)] // FastMatrix is indexed using a tuple of (row, column).
/// 
/// assert_eq!(element, 6)
/// ```
pub struct FastMatrix<T> {
    pub(crate) pointer: *mut T,
    pub rows: usize,
    pub columns: usize,
}

impl<T> FastMatrix<T> {
    /// ## Info
    /// makes a new empty [`FastMatrix`].
    /// 
    /// ## Warning
    /// every element of this array will be a null pointer when read if you don't write it first, so use with caution.
    /// 
    /// ## Panics
    /// if `rows == 0` or `columns == 0`.
    #[inline(always)]
    pub unsafe fn new_empty(rows: usize, columns: usize) -> FastMatrix<T> {
        assert_ne!(rows, 0, "FastMatrix: rows cannot be 0!");
        assert_ne!(columns, 0, "FastMatrix: columns cannot be 0!");

        let layout = Layout::from_size_align(rows*columns * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let pointer = unsafe {
            alloc_zeroed(layout) as *mut T
        };

        if pointer.is_null() {
            handle_alloc_error(layout);
        }

        FastMatrix { pointer, rows, columns }
    }

    /// ## Info
    /// this method has the same functionality as [`FastMatrix::new_empty`], but skips the `rows != 0` and `columns != 0` check for performance reasons.
    ///
    /// if either `row == 0` or `columns == 0`, using this method becomes undefined behavior
    #[inline(always)]
    pub unsafe fn new_empty_unchecked(rows: usize, columns: usize) -> FastMatrix<T> {
        // assert_ne!(rows, 0, "FastMatrix: rows cannot be 0!");
        // assert_ne!(columns, 0, "FastMatrix: columns cannot be 0!");

        let layout = Layout::from_size_align(rows*columns * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let pointer = unsafe {
            alloc_zeroed(layout) as *mut T
        };

        if pointer.is_null() {
            handle_alloc_error(layout);
        }

        FastMatrix { pointer, rows, columns }
    }

    #[inline]
    /// ## Info
    /// creates a new [`FastMatrix`] from a closure or function that accepts a tuple of (`row`, `column`) that's being set.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::FastMatrix;
    /// use fast_array::fast_matrix;
    /// 
    /// let func = |(_, column)| {
    ///     column
    /// };
    /// 
    /// let fast_matrix = FastMatrix::new_func(3, 3, func);
    /// assert_eq!(fast_matrix, fast_matrix!([0,1,2], [0,1,2], [0,1,2]));
    /// ```
    pub fn new_func<F>(rows: usize, columns: usize, mut func: F) -> FastMatrix<T> 
    where 
        F: FnMut((usize, usize)) -> T
    {
        assert_ne!(rows, 0, "FastMatrix: rows cannot be 0!");
        assert_ne!(rows, 0, "FastMatrix: columns cannot be 0!");

        let layout = Layout::from_size_align(rows*columns * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let pointer = unsafe {
            alloc_zeroed(layout) as *mut T
        };

        if pointer.is_null() {
            handle_alloc_error(layout);
        }

        for row_i in 0..rows {
            for col_i in 0..columns {
                unsafe { *pointer.add(row_i * columns + col_i) = func((row_i, col_i)) }
            }
        }

        FastMatrix { pointer, rows, columns }
    }

    #[inline]
    /// ## Info
    /// does the same thing [`FastMatrix::new_func`] does, but skips the checks on `rows` and `columns` for performance reasons.
    /// 
    /// if either `rows` or `columns` are equal to 0, using this method is undefined behavior.
    pub unsafe fn new_func_unchecked<F>(rows: usize, columns: usize, mut func: F) -> FastMatrix<T> 
    where 
        F: FnMut((usize, usize)) -> T
    {
        // assert_ne!(rows, 0, "FastMatrix: rows cannot be 0!");
        // assert_ne!(rows, 0, "FastMatrix: columns cannot be 0!");

        let layout = Layout::from_size_align(rows*columns * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let pointer = unsafe {
            alloc_zeroed(layout) as *mut T
        };

        if pointer.is_null() {
            handle_alloc_error(layout);
        }

        for row_i in 0..rows {
            for col_i in 0..columns {
                unsafe { *pointer.add(row_i * columns + col_i) = func((row_i, col_i)) }
            }
        }

        FastMatrix { pointer, rows, columns }
    }
    
    #[inline(always)]
    /// ## Info
    /// makes an extremely cheap conversion from [`FastMatrix`] to [`FastIterator`], consuming self.
    /// the rows in [`FastMatrix`] get concatenated one after the other in [`FastIterator`].
    /// 
    /// ## Example 
    /// ```
    /// use fast_arr::fast_matrix;
    /// 
    /// let fast_matrix = fast_matrix!([1,2,3], [4,5,6], [7,8,9]);
    /// 
    /// let iter = fast_matrix.into_fast_iterator();
    /// let iter2 = fast_arr!(1,2,3,4,5,6,7,8,9).into_fast_iterator();
    /// 
    /// assert_eq!(iter, iter2);
    /// ```
    pub fn into_fast_iter(mut self) -> FastIterator<T> {
        let pointer = self.pointer;
        self.pointer = ptr::null_mut();
        FastIterator { pointer, len: self.columns*self.rows, current_index: (0, 0) }
    }

    /// ## Info 
    /// creates a new [`FastIterator`] holding a reference to each element of [`FastMatrix`] in order, from left to right and from top to bottom.
    pub fn iter(&self) -> FastIterator<&T> {
        let mut index = 0;
        let func = || {
            index+=1;
            unsafe { &*self.pointer.add(index-1) }
        };
        unsafe { FastIterator::new_func_unchecked(self.columns*self.rows, func) }
    }

        /// ## Info 
    /// creates a new [`FastIterator`] holding a mutable reference to each element of [`FastMatrix`] in order, from left to right and from top to bottom.
    pub fn iter_mut(&mut self) -> FastIterator<&mut T> {
        let mut index = 0;
        let func = || {
            index+=1;
            unsafe { &mut *self.pointer.add(index-1) }
        };
        unsafe { FastIterator::new_func_unchecked(self.columns*self.rows, func) }
    }
}

impl<T: Clone> FastMatrix<T> {
    #[inline(always)]
    /// ## Info
    /// creates a new [`FastMatrix`] filling it with `fill_value`.
    pub fn new(rows: usize, columns: usize, fill_value: T) -> FastMatrix<T> {
        assert_ne!(rows, 0, "FastMatrix: rows cannot be 0!");
        assert_ne!(columns, 0, "FastMatrix: columns cannot be 0!");
    
        let layout = Layout::from_size_align(rows*columns * std::mem::size_of::<T>(), 32).expect("failed to create layout");
    
        let pointer = unsafe {
            alloc_zeroed(layout) as *mut T
        };
    
        if pointer.is_null() {
            panic!("FastMatrix allocation failed!");
        }

        // println!("Allocating FastMatrix at {:p} (size: {})", pointer, rows*columns);
    
        for i in 0..rows*columns {
            unsafe { pointer.add(i).write(fill_value.clone()) }
        }
        // println!("mnmn");
        FastMatrix { pointer, rows, columns }
    }

    #[inline(always)]
    /// ## Info
    /// does the same thing as [`FastMatrix::new`], but skips the checks on `rows` and `columns` for performance reasons.
    /// 
    /// if either one of `rows` or `columns` is 0, using the method becomes undefined behavior.
    pub fn new_unchecked(rows: usize, columns: usize, fill_value: T) -> FastMatrix<T> {
        // assert_ne!(rows, 0, "FastMatrix: rows cannot be 0!");
        // assert_ne!(columns, 0, "FastMatrix: columns cannot be 0!");
    
        let layout = Layout::from_size_align(rows*columns * std::mem::size_of::<T>(), 32).expect("failed to create layout");
    
        let pointer = unsafe {
            alloc_zeroed(layout) as *mut T
        };
    
        if pointer.is_null() {
            handle_alloc_error(layout);
        }
    
        for i in 0..rows*columns {
            unsafe { *pointer.add(i) = fill_value.clone() }
        }
    
        FastMatrix { pointer, rows, columns }
    }

    #[inline(always)]
    /// ## Info 
    /// turns [`FastMatrix`] into nested [`FastArray`]s.
    pub fn into_nested_arrays(self) -> FastArray<FastArray<T>> {
        let mut fast_arr_outer = unsafe { FastArray::new_empty(self.rows) };
        
        for i in 0..self.rows {
            fast_arr_outer[i] = self.get_row(i);
            // println!("xxx4");
        }

        fast_arr_outer
    }

    pub fn into_fast_iter_arrays(self) -> FastIterator<FastArray<T>> {
        // println!("xxx3");
        self.into_nested_arrays().into_fast_iterator()
    }
}

impl<T: Default> FastMatrix<T> {
    /// ## Info
    /// creates a new [`FastMatrix`] of the given rows and columns and fills it with the [`Default`] value of the type T.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::FastMatrix;
    /// use fast_array::fast_matrix;
    /// 
    /// let fast_matrix = FastMatrix::new_default(2, 3);
    /// 
    /// assert_eq!(fast_matrix, fast_matrix!([0,0,0], [0,0,0]));
    /// ```
    /// 
    /// ## Panics
    /// panics if either `rows == 0` or `columns == 0`.
    #[inline]
    pub fn new_default(rows: usize, columns: usize) -> FastMatrix<T> {
        assert!(rows != 0 && columns != 0, "rows and columns cannot be 0!");

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(rows*columns * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        unsafe {
            for i in 0..rows*columns {
                raw_ptr.add(i).write(T::default());
            };
        };

        FastMatrix {
            pointer: raw_ptr,
            rows,
            columns
        }
    }

    #[inline]
    /// ## Info 
    /// same as [`FastMatrix::new_default`], just doesn't do checks on `rows` and `columns` for performance reasons.
    /// 
    /// if `rows == 0` or `columns == 0`, using the method is undefined behavior. 
    pub unsafe fn new_default_unchecked(rows: usize, columns: usize) -> FastMatrix<T> {
        assert!(rows != 0 && columns != 0, "rows and columns cannot be 0!");

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(rows*columns * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        unsafe {
            for i in 0..rows*columns {
                raw_ptr.add(i).write(T::default());
            };
        };

        FastMatrix {
            pointer: raw_ptr,
            rows,
            columns
        }
    }
}


impl<T> Index<(usize, usize)> for FastMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        // println!("xxx6");
        assert!(index.0 < self.rows && index.1 < self.columns);
        unsafe { &*self.pointer.add(index.0 * self.columns + index.1) }
    }
}

impl<T> IndexMut<(usize, usize)> for FastMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.rows && index.1 < self.columns);
        unsafe { &mut *self.pointer.add(index.0 * self.columns + index.1) }
    }
}

impl<T> IntoIterator for FastMatrix<T> {
    type Item = T;

    type IntoIter = FastIterator<T>;

    #[doc(alias = "into_fast_iter")]
    fn into_iter(self) -> Self::IntoIter {
        self.into_fast_iter()
    }
}

impl<T> Drop for FastMatrix<T> {
    fn drop(&mut self) {
        // println!("Dropping FastMatrix at {:p} (size: {} x {})", self.pointer, self.rows, self.columns);
        if !self.pointer.is_null() {
            let size = self.rows * self.columns;
            let layout = Layout::array::<T>(size).expect("Failed to create layout");

            unsafe {
                // println!("Deallocating memory at {:p} (size: {})", self.pointer, size);
                std::alloc::dealloc(self.pointer as *mut u8, layout);
                self.pointer = std::ptr::null_mut(); // Prevent double free
            }
        }
    }
}
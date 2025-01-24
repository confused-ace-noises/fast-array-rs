use std::{
    ops::{AddAssign, Div, DivAssign, Mul, SubAssign},
    ptr,
};

use crate::FastMatrix;

impl<T> FastMatrix<T> {
    // ------- ROWS --------
    /// ## Info
    /// swaps two rows.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_matrix;
    /// 
    /// let mut fast_matrix = fast_matrix!([1,2,3], [4,5,6], [7,8,9]);
    /// fast_matrix.swap_rows(0, 2);
    /// 
    /// assert_eq!(fast_matrix, fast_matrix!([7,8,9], [4,5,6], [1,2,3]))
    /// ```
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        assert!(
            row1 < self.rows && row2 < self.rows,
            "FastMatrix: tried to index out of bounds."
        );

        for i in 0..self.columns {
            unsafe { self.swap_unchecked((row1, i), (row2, i)) };
        }
    }
    
    /// ## Info 
    /// same as [`FastMatrix::swap_rows`], but doesn't check for out of bounds conditions for performance reasons.
    /// 
    /// if `row1` or `row2` is out of bounds, using the method is undefined behavior. 
    pub unsafe fn swap_rows_unchecked(&mut self, row1: usize, row2: usize) {
        // assert!(row1<self.rows && row2 < self.rows, "FastMatrix: tried to index out of bounds.");

        for i in 0..self.columns {
            self.swap_unchecked((row1, i), (row2, i));
        }
    }

    /// ## WARNING
    /// **the use of this method is HIGHLY discouraged as it doesn't respect borrow checker rules**
    /// ## Info 
    /// same as [`FastMatrix::swap_rows`], but borrows immutably self even if it modifies it.
    /// 
    /// if any other operations are done on the [`FastMatrix`] while the method is running, using the method is undefined behavior. 
    pub unsafe fn swap_rows_unsafe(&self, row1: usize, row2: usize) {
        assert!(
            row1 < self.rows && row2 < self.rows,
            "FastMatrix: tried to index out of bounds."
        );

        for i in 0..self.columns {
            self.swap_unchecked_unsafe((row1, i), (row2, i));
        }
    }

    /// ## WARNING
    /// **the use of this method is HIGHLY discouraged as it doesn't respect borrow checker rules**
    /// ## Info 
    /// same as [`FastMatrix::swap_rows`], but borrows immutably self even if it modifies it, and also skips the out of bounds condition checks on `row1` and `row2`.
    /// 
    /// if any other operations are done on the [`FastMatrix`] while the method is running, or if `row1` or `row2` are out of bounds, using the method is undefined behavior. 
    pub unsafe fn swap_rows_unchecked_unsafe(&self, row1: usize, row2: usize) {
        // assert!(row1<self.rows && row2 < self.rows, "FastMatrix: tried to index out of bounds.");

        for i in 0..self.columns {
            self.swap_unchecked_unsafe((row1, i), (row2, i));
        }
    }

    // ------- ROWS --------
    // ------- COLS --------

    /// ## Info
    /// swaps two columns.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_matrix;
    /// 
    /// let mut fast_matrix = fast_matrix!([1,2,3], [4,5,6], [7,8,9]);
    /// fast_matrix.swap_columns(0, 2);
    /// 
    /// assert_eq!(fast_matrix, fast_matrix!([3,2,1], [6,5,4], [9,8,7]));
    /// ```
    pub fn swap_columns(&mut self, column1: usize, column2: usize) {
        assert!(
            column1 < self.columns && column2 < self.rows,
            "FastMatrix: tried to index out of bounds."
        );

        for i in 0..self.rows {
            unsafe { self.swap_unchecked((i, column1), (i, column2)) };
        }
    }

    /// ## Info 
    /// same as [`FastMatrix::swap_columns`], but doesn't check for out of bounds conditions for performance reasons.
    /// 
    /// if `column1` or `column2` is out of bounds, using the method is undefined behavior. 
    pub unsafe fn swap_columns_unchecked(&mut self, column1: usize, column2: usize) {
        // assert!(row1<self.rows && row2 < self.rows, "FastMatrix: tried to index out of bounds.");

        for i in 0..self.rows {
            self.swap_unchecked((i, column1), (i, column2));
        }
    }

        /// ## WARNING
        /// **the use of this method is HIGHLY discouraged as it doesn't respect borrow checker rules**
        /// ## Info 
        /// same as [`FastMatrix::swap_columns`], but borrows immutably self even if it modifies it.
        /// 
        /// if any other operations are done on the [`FastMatrix`] while the method is running, using the method is undefined behavior. 
    pub unsafe fn swap_columns_unsafe(&self, column1: usize, column2: usize) {
        assert!(
            column1 < self.columns && column2 < self.rows,
            "FastMatrix: tried to index out of bounds."
        );

        for i in 0..self.rows {
            self.swap_unchecked_unsafe((i, column1), (i, column2));
        }
    }

    /// ## WARNING
    /// **the use of this method is HIGHLY discouraged as it doesn't respect borrow checker rules**
    /// ## Info 
    /// same as [`FastMatrix::swap_columns`], but borrows immutably self even if it modifies it, and also skips the out of bounds condition checks on `column1` and `column2`.
    /// 
    /// if any other operations are done on the [`FastMatrix`] while the method is running, or if `column1` or `column2` are out of bounds, using the method is undefined behavior. 
    pub unsafe fn swap_columns_unchecked_unsafe(&self, column1: usize, column2: usize) {
        // assert!(row1<self.rows && row2 < self.rows, "FastMatrix: tried to index out of bounds.");

        for i in 0..self.rows {
            self.swap_unchecked_unsafe((i, column1), (i, column2));
        }
    }
}

impl<T: Clone> FastMatrix<T> {
    #[inline(always)]
    pub fn transpose(&self) -> FastMatrix<T> {
        let func = |(row, col)| {
            self[(col, row)].clone()
        };

        unsafe { FastMatrix::new_func_unchecked(self.columns, self.rows, func) }
    }
}

impl<T> FastMatrix<T>
where
    T: Clone
        + Default
        + PartialOrd
        + Mul<Output = T>
        + Div<Output = T>
        + DivAssign
        + SubAssign
        + AddAssign
        + TryFrom<f64>,

    <T as TryFrom<f64>>::Error: std::fmt::Debug
{
    /// ## Info
    /// returns the determinant of self.
    /// even if `T` is required to implement `TryFrom<f64>`, the method will panic if the type couldn't be converted to `f64`.
    /// this is done to enable support of primary types for the method.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_matrix;
    /// 
    /// let fast_matrix: FastMatrix<f64> = fast_matrix!([1.0, 4.0], [2.0, 3.0]);
    /// 
    /// assert_eq!(fast_matrix.determinant(), -5.0);
    /// ```
    pub fn determinant(&self) -> T {
        assert!(self.rows == self.columns, "Matrix must be square");
        let n = self.rows;

        let mat = self.clone();
        let mut det = T::try_from(1.0).expect("FastMatrix: determinant: couldn't convert T to f64");
        let mut sign = T::try_from(1.0).expect("FastMatrix: determinant: couldn't convert T to f64");

        for k in 0..n {
            let mut pivot = k;
            for i in k + 1..n {
                unsafe {
                    if *mat.pointer.add(i * n + k) > *mat.pointer.add(pivot * n + k) {
                        pivot = i;
                    }
                }
            }

            if pivot != k {
                for j in 0..n {
                    unsafe { ptr::swap(mat.pointer.add(pivot * n + j), mat.pointer.add(k * n + j)) };
                }
                sign = sign * T::try_from(-1.0).expect("FastMatrix: determinant: couldn't convert T to f64");
            }

            let pivot_value = unsafe { mat.pointer.add(k * n + k).read() };
            if pivot_value == T::try_from(0.0).expect("FastMatrix: determinant: couldn't convert T to f64") {
                return T::try_from(0.0).expect("FastMatrix: determinant: couldn't convert T to f64");
            }

            det = det * pivot_value.clone();

            for i in k + 1..n {
                let factor = unsafe { mat.pointer.add(i * n + k).read() } / pivot_value.clone();
                for j in k..n {
                    unsafe{*mat.pointer.add(i * n + j) -= factor.clone() * mat.pointer.add(k * n + j).read()};
                }
            }
        }

        det * sign
    }
}
use crate::FastArray;
use std::fmt::Display;
use std::ptr;

use super::fast_matrix::FastMatrix;

impl<T: Display> Display for FastMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;

        let mut is_first_outer = true;

        // TODO: use iters
        for row in 0..self.rows {
            if is_first_outer {
                f.write_str("\n    [")?;
            } else {
                f.write_str(",\n    [")?
            }
            is_first_outer = false;
            let mut is_first_inner = true;
            for column in 0..self.columns {
                if is_first_inner {
                    write!(f, "{}", &self[(row, column)])?;
                } else {
                    write!(f, ", {}", &self[(row, column)])?;
                }

                is_first_inner = false
            }

            f.write_str("]")?;
        }

        f.write_str("\n]")
    }
}

impl<T: PartialEq> PartialEq for FastMatrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T: Eq> Eq for FastMatrix<T> {}

impl<T> FastMatrix<T> {
    #[inline(always)]
    /// ## Info
    /// swaps two values.
    ///
    /// ## Example
    /// ```
    /// use fast_array::fast_matrix;
    ///
    /// let mut fast_matrix = fast_matrix!([1,2,3], [4,5,6]);
    /// fast_matrix.swap((0,0), (1,2));
    ///
    /// assert_eq!(fast_matrix, fast_matrix!([6,2,3], [4,5,1]));
    /// ```
    pub fn swap(&mut self, index1: (usize, usize), index2: (usize, usize)) {
        assert!(
            self.rows > index1.0 && self.rows > index2.0,
            "FastMatrix: swap: tried to index out of bounds."
        );
        assert!(
            self.columns > index1.1 && self.columns > index2.1,
            "FastMatrix: swap: tried to index out of bounds."
        );

        unsafe {
            ptr::swap(
                self.get_pointer_mut_unchecked(index1),
                self.get_pointer_mut_unchecked(index2),
            )
        };
    }

    #[inline(always)]
    /// does the same thing as [`FastMatrix::swap`], but doesn't make out of bounds condition checks on `index1` and `index2`
    ///
    /// if either one of `index1` or `index2` are out of bounds, using this function is undefined behavior.
    pub unsafe fn swap_unchecked(&mut self, index1: (usize, usize), index2: (usize, usize)) {
        // assert!(self.rows>index1.0 && self.rows>index2.0, "FastMatrix: swap: tried to index out of bounds.");
        // assert!(self.columns>index1.1 && self.columns>index2.1, "FastMatrix: swap: tried to index out of bounds.");

        unsafe {
            ptr::swap(
                self.get_pointer_mut_unchecked(index1),
                self.get_pointer_mut_unchecked(index2),
            )
        };
    }

    #[inline(always)]
    /// ## WARNING
    /// **the use of this method is highly discouraged as it breaks the borrow checker rules.**
    ///
    /// ## Info
    /// does the same thing as [`FastMatrix::swap`], but borrows self immutably even if it modifies self.
    ///
    /// if any other operation gets carried out on [`FastMatrix`] while this method is running, using this method is undefined behavior.
    pub unsafe fn swap_unsafe(&self, index1: (usize, usize), index2: (usize, usize)) {
        assert!(
            self.rows > index1.0 && self.rows > index2.0,
            "FastMatrix: swap: tried to index out of bounds."
        );
        assert!(
            self.columns > index1.1 && self.columns > index2.1,
            "FastMatrix: swap: tried to index out of bounds."
        );

        unsafe {
            ptr::swap(
                self.get_pointer_unchecked(index1).cast_mut(),
                self.get_pointer_unchecked(index2).cast_mut(),
            )
        };
    }

    #[inline(always)]
    /// ## WARNING
    /// **the use of this method is highly discouraged as it breaks the borrow checker rules.**
    ///
    /// ## Info
    /// does the same thing as [`FastMatrix::swap`], but borrows self immutably even if it modifies self, and doesn't make out of bounds condition checks on `index1` and `index2`
    ///
    /// if any other operation gets carried out on [`FastMatrix`] while this method is running or if either one of `index1` or `index2` are out of bounds, using this function is undefined behavior.
    pub unsafe fn swap_unchecked_unsafe(&self, index1: (usize, usize), index2: (usize, usize)) {
        // assert!(self.rows>index1.0 && self.rows>index2.0, "FastMatrix: swap: tried to index out of bounds.");
        // assert!(self.columns>index1.1 && self.columns>index2.1, "FastMatrix: swap: tried to index out of bounds.");

        unsafe {
            ptr::swap(
                self.get_pointer_unchecked(index1).cast_mut(),
                self.get_pointer_unchecked(index2).cast_mut(),
            )
        };
    }
}

impl<T> FastMatrix<T> {
    #[inline(always)]
    /// ## Info
    /// gets the pointer to the given `index`.
    pub fn get_pointer(&self, index: (usize, usize)) -> *const T {
        assert!(
            self.rows > index.0,
            "FastMatrix: tried to index out of bounds."
        );
        assert!(
            self.columns > index.1,
            "FastMatrix: tried to index out of bounds."
        );

        unsafe { self.pointer.add(index.calc_index(self.columns)) }
    }

    #[inline(always)]
    /// ## Info
    /// does the same thing as [`FastMatrix::get_pointer`], but doesn't check for out of bounds conditions.
    ///
    /// if `index` is out of bounds, using this method is undefined behavior.
    pub unsafe fn get_pointer_unchecked(&self, index: (usize, usize)) -> *const T {
        unsafe { self.pointer.add(index.calc_index(self.columns)) }
    }

    #[inline(always)]
    /// ## Info
    /// gets the mutable pointer to the given `index`.
    pub fn get_pointer_mut(&mut self, index: (usize, usize)) -> *mut T {
        assert!(
            self.rows > index.0,
            "FastMatrix: tried to index out of bounds."
        );
        assert!(
            self.columns > index.1,
            "FastMatrix: tried to index out of bounds."
        );

        unsafe { self.pointer.add(index.calc_index(self.columns)) }
    }

    #[inline(always)]
    /// ## Info
    /// does the same thing as [`FastMatrix::get_pointer_mut`], but doesn't check for out of bounds conditions.
    ///
    /// if `index` is out of bounds, using this method is undefined behavior.
    pub unsafe fn get_pointer_mut_unchecked(&mut self, index: (usize, usize)) -> *mut T {
        unsafe { self.pointer.add(index.calc_index(self.columns)) }
    }

    /// ## Warning
    /// **the use of this method is highly discouraged, as it breaks the borrow checker rules**
    ///
    /// ## Info
    /// does the same thing as [`FastMatrix::get_pointer_mut`], but borrows self as immutable even if it returns a mutable pointer.
    #[inline(always)]
    pub unsafe fn get_pointer_mut_unsafe(&self, index: (usize, usize)) -> *mut T {
        assert!(
            self.rows > index.0,
            "FastMatrix: tried to index out of bounds."
        );
        assert!(
            self.columns > index.1,
            "FastMatrix: tried to index out of bounds."
        );

        unsafe { self.pointer.add(index.calc_index(self.columns)) }
    }

    #[inline(always)]
    /// ## Warning
    /// **the use of this method is highly discouraged, as it breaks the borrow checker rules**
    ///
    /// ## Info
    /// does the same thing as [`FastMatrix::get_pointer_mut`], but borrows self as immutable even if it returns a mutable pointer and skips the out of bounds condition checks on `index`.
    ///
    /// if `index` is out of bounds, using this method is undefined behavior.
    pub unsafe fn get_pointer_mut_unchecked_unsafe(&self, index: (usize, usize)) -> *mut T {
        unsafe { self.pointer.add(index.calc_index(self.columns)) }
    }
}

impl<T: Clone> FastMatrix<T> {
    #[inline(always)]
    /// ## Info
    /// gets a given `row` of the [`FastMatrix`] by cloning it to a [`FastArray`].
    ///
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    /// use fast_array::fast_matrix;
    ///
    /// let fast_matrix = fast_matrix!([1,2,3], [4,5,6], [7,8,9]);
    ///
    /// assert_eq!(fast_matrix.get_row(0), fast_arr!(1,2,3));
    /// ```
    pub fn get_row(&self, row: usize) -> FastArray<T> {
        assert!(self.rows > row, "FastMatrix: tried to index out of bounds.");
        let func = |index| (&self[(row, index)]).clone();
        unsafe { FastArray::new_func_unchecked(self.columns, func) }
    }

    #[inline(always)]
    /// ## Info
    /// does the same thing as [`FastMatrix::get_row`], but doesn't make out of bounds checks on `row`.
    ///
    /// if `row` is out of bounds, using this method is undefined behavior.
    pub unsafe fn get_row_unchecked(&self, row: usize) -> FastArray<T> {
        // assert!(self.rows>row, "FastMatrix: tried to index out of bounds.");
        let func = |index| (&self[(row, index)]).clone();
        unsafe { FastArray::new_func_unchecked(self.columns, func) }
    }

    #[inline(always)]
    /// ## Info
    /// gets a given `column` of the [`FastMatrix`] by cloning it to a [`FastArray`].
    ///
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    /// use fast_array::fast_matrix;
    ///
    /// let fast_matrix = fast_matrix!([1,2,3], [4,5,6], [7,8,9]);
    ///
    /// assert_eq!(fast_matrix.get_row(0), fast_arr!(1,4,7));
    /// ```
    pub fn get_column(&self, column: usize) -> FastArray<T> {
        assert!(
            self.columns > column,
            "FastMatrix: tried to index out of bounds."
        );
        let func = |index| (&self[(index, column)]).clone();
        unsafe { FastArray::new_func_unchecked(self.columns, func) }
    }

    #[inline(always)]
    /// ## Info
    /// does the same thing as [`FastMatrix::get_column`], but doesn't make out of bounds checks on `column`.
    ///
    /// if `column` is out of bounds, using this method is undefined behavior.
    pub fn get_column_unchecked(&self, column: usize) -> FastArray<T> {
        // assert!(self.rows>row, "FastMatrix: tried to index out of bounds.");
        let func = |index| (&self[(index, column)]).clone();
        unsafe { FastArray::new_func_unchecked(self.columns, func) }
    }
}

trait CalcIndex {
    fn calc_index(&self, offset: usize) -> usize;
}

impl CalcIndex for (usize, usize) {
    fn calc_index(&self, offset: usize) -> usize {
        self.0 * offset + self.1
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize + Clone> serde::Serialize for FastMatrix<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_seq(Some(self.rows*self.columns))?;
        
        let mut iter = self.clone().into_fast_iter_arrays();
        
        while let Some(row) = iter.next() {
            serde::ser::SerializeSeq::serialize_element(&mut state, &row)?;
        }

        serde::ser::SerializeSeq::end(state)
    }
}

#[cfg(feature = "serde")]
use std::marker::PhantomData;
#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de> + Clone> serde::Deserialize<'de> for FastMatrix<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct FastMatrixVisitor<T> {
            phant: PhantomData<T>
        }

        impl<'de, U: serde::Deserialize<'de> + Clone> serde::de::Visitor<'de> for FastMatrixVisitor<U> {
            type Value = FastMatrix<U>;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting an array of arrays.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>, 
            {
                // let _len = seq.size_hint().unwrap_or({
                    let mut vec = Vec::new();

                    while let Some(item) = seq.next_element()? {
                        vec.push(item);
                    }

                    return Ok(vec.into())
                // });

                // if _len == 1 {
                //     let mut vec = Vec::new();

                //     while let Some(item) = seq.next_element()? {
                //         vec.push(item);
                //     }

                //     return Ok(vec.into())
                // }

                // let iter = |_| seq.next_element::<U>();

                // FastMatrix::new_func(_len, iter);
            }
        }

        deserializer.deserialize_seq(FastMatrixVisitor::<T> { phant: PhantomData::<T>})
    }
}

impl<T: Clone> From<Vec<Vec<T>>> for FastMatrix<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let rows = value.len();

        let mut first = true;
        let mut first_val = 0;

        value.iter().for_each(|x| {
            if first {
                first = false;
                first_val = x.len();
            } else {
                assert_eq!(first_val, x.len(), "FastMatrix: the inner vectors aren't all the same size");
            }
        });

        let columns = first_val;

        let mut fast_matrix: FastMatrix<T> = unsafe {
            FastMatrix::new_empty(rows, columns)
        };

        for x in 0..rows {
            for y in 0..columns {
                fast_matrix[(x,y)] = value.clone()[x][y].clone()
            }
        }

        fast_matrix
    }
}
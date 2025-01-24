use std::{
    alloc::{alloc, dealloc, Layout},
    ptr,
};

#[derive(Debug, Clone)]
#[repr(align(32))]
/// ## Info
/// a very fast and bare-bones iterator.
pub struct FastIterator<T> {
    pub(crate) pointer: *mut T,
    pub(crate) len: usize,
    pub(crate) current_index: (usize, usize),
}
impl<T> FastIterator<T> {
    /// ## Info
    /// **ONLY** allocates the memory required by the iterator, but doesn't actually fill it with any values.
    /// ## Warning
    /// every pointer in a [`FastIterator`] created with this method will point to null data until written.
    pub unsafe fn allocate_mem(len: usize) -> FastIterator<T> {
        assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32)
            .expect("failed to create layout");

        let raw_ptr = unsafe { alloc(layout) as *mut T };

        if raw_ptr.is_null() {
            panic!("Memory alloc failed.")
        };

        FastIterator {
            pointer: raw_ptr,
            len: len,
            current_index: (0, 0),
        }
    }

    /// ## Info
    /// this method has the same functionality as [`FastIterator::allocate_mem`], but skips the `len != 0` check for performance reasons.
    ///
    /// If `len == 0`, using this method becomes undefined behavior
    pub unsafe fn allocate_mem_unchecked(len: usize) -> FastIterator<T> {
        // assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32)
            .expect("failed to create layout");

        let raw_ptr = unsafe { alloc(layout) as *mut T };

        if raw_ptr.is_null() {
            panic!("Memory alloc failed.")
        };

        FastIterator {
            pointer: raw_ptr,
            len: len,
            current_index: (0, 0),
        }
    }

    /// ## Info
    /// creates a new [`FastIterator`] from a function and/or closure by calling it to fill every element of the iterator.
    ///
    /// ## Example
    /// ```
    /// use fast_array::{FastArray, FastIterator};
    ///
    /// let mut val = 0;
    /// let func = || {
    ///     val+=1;
    ///     val
    /// };
    ///
    /// let fast_iter = FastIterator::new_func(5, func);
    /// let fast_arr = fast_iter.as_fast_array();
    /// assert_eq!(fast_arr.to_string(), "[1, 2, 3, 4, 5]");
    /// ```
    ///
    /// ## Panics
    /// if len == 0.
    pub fn new_func<F>(len: usize, mut func: F) -> FastIterator<T>
    where
        F: FnMut() -> T,
    {
        assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32)
            .expect("failed to create layout");

        let raw_ptr = unsafe { alloc(layout) as *mut T };

        if raw_ptr.is_null() {
            panic!("Memory alloc failed.")
        };

        for x in 0..len {
            unsafe { raw_ptr.add(x).write(func()) };
        }

        FastIterator {
            pointer: raw_ptr,
            len: len,
            current_index: (0, 0),
        }
    }

    /// ## Info
    /// this method has the same functionality as [`FastIterator::new_func`], just skips the `len != 0` check for performance reasons.
    /// if `len == 0`, using this function is undefined behavior
    pub unsafe fn new_func_unchecked<F>(len: usize, mut func: F) -> FastIterator<T>
    where
        F: FnMut() -> T,
    {
        // assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32)
            .expect("failed to create layout");

        let raw_ptr = unsafe { alloc(layout) as *mut T };

        if raw_ptr.is_null() {
            panic!("Memory alloc failed.")
        };

        for x in 0..len {
            unsafe { raw_ptr.add(x).write(func()) };
        }

        FastIterator {
            pointer: raw_ptr,
            len: len,
            current_index: (0, 0),
        }
    }
}

unsafe impl<T> Send for FastIterator<T> {}
// unsafe impl<T> Sync for FastIterator<T> {}

// #[cfg(not(feature = "rayon"))]
mod iter {
    use super::*;
    impl<T /*: Display+Clone*/> Iterator for FastIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_index.0 + self.current_index.1 >= self.len {
                None
            } else {
                let read_ptr = unsafe { ptr::read(self.pointer) };
                // println!("*{}", unsafe{ (*self.pointer).clone() });
                self.pointer = unsafe { self.pointer.add(1) };
                self.current_index.0 += 1;
                Some(read_ptr)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.len, Some(self.len))
        }
    }
}

impl<T> DoubleEndedIterator for FastIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // println!("{}", 3);
        if self.current_index.0 + self.current_index.1 >= self.len {
            None
        } else {
            let read_ptr = unsafe { ptr::read(self.pointer.add(self.len -1 -self.current_index.0)) };
            // println!("*{}", unsafe{ (*self.pointer).clone() });
            // self.pointer = unsafe { self.pointer.add(1) };
            self.current_index.1 += 1;
            Some(read_ptr)
        }
    }
}

impl<T> Drop for FastIterator<T> {
    fn drop(&mut self) {
        let len = (self.len - self.current_index.0) - self.current_index.1;

        for i in 0..len {
            // println!("{}", i);
            unsafe {
                ptr::drop_in_place(self.pointer.add(i));
            }
        }

        unsafe {
            // Calculate the original pointer from the current pointer and index
            let original_pointer = self.pointer.sub(self.current_index.0);

            let layout = Layout::array::<T>(self.len).expect("Failed to create layout");

            dealloc(original_pointer as *mut u8, layout); // Deallocate memory
        }
    }
}

impl<T> ExactSizeIterator for FastIterator<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }
}

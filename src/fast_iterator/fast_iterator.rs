

use std::{
    alloc::{alloc, dealloc, Layout}, hint::black_box, ops::{Range, RangeInclusive}, ptr, time::Instant
};

use crate::FastArray;

#[derive(Debug, Clone)]
#[repr(align(32))]
/// ## Info
/// a very fast and bare-bones iterator.
pub struct FastIterator<T> {
    pub(crate) pointer: *mut T,
    pub(crate) len: usize,
    pub(crate) current_index: usize,
}
impl<T> FastIterator<T> {
    /// ## Info
    /// **ONLY** allocates the memory required by the iterator, but doesn't actually fill it with any values.
    /// ## Warning
    /// every pointer in a [`FastIterator`] created with this method will point to null data until written.
    pub unsafe fn allocate_mem(len: usize) -> FastIterator<T> {
        assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let raw_ptr = unsafe { alloc(layout) as *mut T };

        if raw_ptr.is_null() {
            panic!("Memory alloc failed.")
        };

        FastIterator {
            pointer: raw_ptr,
            len: len,
            current_index: 0,
        }
    }

    /// ## Info
    /// this method has the same functionality as [`FastIterator::allocate_mem`], but skips the `len != 0` check for performance reasons.
    /// 
    /// If `len == 0`, using this method becomes undefined behavior
    pub unsafe fn allocate_mem_unchecked(len: usize) -> FastIterator<T> {
        // assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let raw_ptr = unsafe { alloc(layout) as *mut T };

        if raw_ptr.is_null() {
            panic!("Memory alloc failed.")
        };

        FastIterator {
            pointer: raw_ptr,
            len: len,
            current_index: 0,
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
        F: FnMut() -> T
    {
        assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let raw_ptr = unsafe { alloc(layout) as *mut T };

        if raw_ptr.is_null() {
            panic!("Memory alloc failed.")
        };

        for x in 0..len {
            unsafe { raw_ptr.add(x).write(func()) };
        };

        FastIterator { pointer: raw_ptr, len: len, current_index: 0 }
    }

    /// ## Info
    /// this method has the same functionality as [`FastIterator::new_func`], just skips the `len != 0` check for performance reasons.
    /// if `len == 0`, using this function is undefined behavior
    pub fn new_func_unchecked<F>(len: usize, mut func: F) -> FastIterator<T> 
    where 
        F: FnMut() -> T
    {
        // assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let raw_ptr = unsafe { alloc(layout) as *mut T };

        if raw_ptr.is_null() {
            panic!("Memory alloc failed.")
        };

        for x in 0..len {
            unsafe { raw_ptr.add(x).write(func()) };
        };

        FastIterator { pointer: raw_ptr, len: len, current_index: 0 }
    }
}

unsafe impl<T> Send for FastIterator<T> {}

impl<T/*: Display+Clone*/> Iterator for FastIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.len {
            None
        } else {
            let read_ptr = unsafe { ptr::read(self.pointer) };
            // println!("*{}", unsafe{ (*self.pointer).clone() });
            self.pointer = unsafe { self.pointer.add(1) };
            self.current_index += 1;
            Some(read_ptr)
        }
    }


    fn size_hint(&self) -> (usize, Option<usize>){
        (self.len, Some(self.len))
    }
}

impl<T> Drop for FastIterator<T> {
    fn drop(&mut self) {
        let len = self.len - self.current_index;

        for i in 0..len {
            println!("{}", i);
            unsafe { ptr::drop_in_place(self.pointer.add(i)); }
        }

        unsafe {
            // Calculate the original pointer from the current pointer and index
            let original_pointer = self.pointer.sub(self.current_index);

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

// #[test]
pub fn test1() {
    let start = Instant::now();
    let vec = (0..=1000).collect::<Vec<_>>();
    let iter = vec.into_iter();
    let mod_vec = iter.map(|x| x+1).collect::<Vec<_>>();
    black_box(mod_vec);
    // println!("{:?}", mod_vec);
    println!("{}", start.elapsed().as_nanos())
}

// #[test]
// pub fn test2() {
//     use crate::fast_array::fast_array_basics::AsFastArray;
//     let start = Instant::now();
//     let fast_arr: FastArray<i32> = (0..=1000).into();
//     let iter = fast_arr.into_iter();
//     let fast_arr = iter.map(|x| x+1).as_fast_array();
//     // fast_arr.simd_add(1);
//     // println!("{}", fast_arr);
//     black_box(fast_arr);
//     println!("{}", start.elapsed().as_nanos())
// }

// #[test]
pub fn vec_iter() {
    let vec = (0..1_000_000).collect::<Vec<_>>();
    let mod_vec: Vec<_> = vec.into_iter().map(|x| x + 1).collect();
    drop(mod_vec);
}

// #[test]
// pub fn fast_arr_iter() {
//     let fast_arr: FastArray<usize> = FastArray::new_range(0, 1_600_000_000);
//     let iter = fast_arr.as_fast_iterator().map(|x| x+1).as_fast_array();
//     drop(iter);
// }

// #[test]
// pub fn fast_arr_simd() {
//     // let mut fast_arr: FastArray<usize> = FastArray::new_range(0, 1_600_000_000);
//     let mut fast_arr: FastArray<usize> = (0..1_000_000).into();
//     fast_arr.simd_add(1);
//     drop(fast_arr);
// }

// use test::{Bencher, black_box};
// #[test]
// pub fn test3() {
//     let start = Instant::now();
//     let mut fast_arr: FastArray<i32> = (0..1600).into();
//     let add: FastArray<i32> = FastArray::new_fast(1600, 1);
//     fast_arr.simd_add_array(&add);
//     println!("{}", fast_arr);
//     println!("{}", start.elapsed().as_micros())
// }

// #[bench]
// pub fn test4(b: &mut Bencher) {
//     b.iter(|| { let mut fast_arr: FastArray<i32> = (0..1600).into();
//     let add: FastArray<i32> = FastArray::new_fast(1600, 1);
//     fast_arr.simd_add_array(&add);})
//     // println!("{}", fast_arr);
//     // println!("{}", start.elapsed().as_micros())
// }
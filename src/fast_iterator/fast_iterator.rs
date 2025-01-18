

use std::{
    alloc::{alloc, dealloc, Layout}, iter::{Inspect, Step}, ops::{Range, RangeInclusive, Sub}, ptr, time::Instant
};

use crate::{fast_arr, fast_array::fast_array_basics::AsFastArray, FastArray};

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
    ///
    pub unsafe fn allocate_mem(len: usize) -> FastIterator<T> {
        assert!(len != 0);

        let layout = Layout::array::<T>(len).expect("failed to create layout");

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

    pub fn new_func<F>(len: usize, mut func: F) -> FastIterator<T> 
    where 
        F: FnMut() -> T
    {
        assert!(len != 0);

        let layout = Layout::array::<T>(len).expect("failed to create layout");

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

unsafe impl<T> Send for FastIterator<T> {
    
}

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

        // Only drop the elements that haven't been moved yet (after current_index).
        for i in 0..len {
            println!("{}", i);
            unsafe { ptr::drop_in_place(self.pointer.add(i)); }
        }

        // Now, deallocate the memory that was used for the array
        unsafe {
            // Calculate the original pointer from the current pointer and index
            let original_pointer = self.pointer.sub(self.current_index);

            // Create the layout to match the original allocation
            let layout = Layout::array::<T>(self.len).expect("Failed to create layout");

            // Deallocate memory using the original pointer
            dealloc(original_pointer as *mut u8, layout); // Deallocate memory
        }
    }
}

impl<T> ExactSizeIterator for FastIterator<T> {
    fn len(&self) -> usize {
        self.len
    }
}

// #[test]
pub fn test1() {
    let start = Instant::now();
    let vec = (0..1600).collect::<Vec<_>>();
    let iter = vec.into_iter();
    let mod_vec = iter.map(|x| x+1).collect::<Vec<_>>();
    println!("{:?}", mod_vec);
    println!("{}", start.elapsed().as_micros())
}

// #[test]
pub fn test2() {
    let start = Instant::now();
    let fast_arr: FastArray<i32> = (0..=1600).into();
    let iter = fast_arr.into_iter();
    let mod_vec = iter.map(|x| x+1).as_fast_array();
    println!("{}", mod_vec);
    println!("{}", start.elapsed().as_micros())
}

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

#[test]
pub fn fast_arr_simd() {
    // let mut fast_arr: FastArray<usize> = FastArray::new_range(0, 1_600_000_000);
    let mut fast_arr: FastArray<usize> = (0..1_000_000).into();
    fast_arr.simd_add(1);
    drop(fast_arr);
}

extern crate test;
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

impl<T: Step+Clone> From<Range<T>> for FastArray<T> {
    #[inline(always)]
    fn from(mut value: Range<T>) -> Self {
        let len = value.clone().count();
        let func = || value.next().unwrap();
        FastArray::new_func(len, func)
    }
}

impl<T: Step+Clone> From<RangeInclusive<T>> for FastArray<T> {
    #[inline(always)]
    fn from(mut value: RangeInclusive<T>) -> Self {
        let len = value.clone().count();
        let func = || value.next().unwrap();
        FastArray::new_func(len, func)
    }
}
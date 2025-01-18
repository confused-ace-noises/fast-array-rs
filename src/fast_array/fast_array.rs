use std::alloc::{alloc, dealloc, Layout};
use std::fmt::Display;
use std::fs::File;
use std::io::{empty, BufReader, Read};
use std::os::unix::fs::MetadataExt;
use std::time::Duration;
use std::{ptr, thread};
use std::ops::{Index, IndexMut};
// use crate::create_unchecked_doc;
use crate::fast_array::fast_array_basics::AsFastArray;
use crate::fast_iterator::fast_iterator::FastIterator;


/// ## Info
/// this is the core struct of the library.
/// it's as bare-bones as an array can be: a pointer, and a length, nothing else.
/// 
/// ## Example
/// ```
/// use fast_array::{fast_arr, FastArray};
/// 
/// let array = fast_arr![1, 2, 3];
/// assert_eq!(array.to_string(), "[1, 2, 3]")
/// ```
#[derive(Debug, Clone)]
#[repr(align(32))]
pub struct FastArray<T> {
    pub(crate) pointer: *mut T,
    pub(crate) size: usize,
}

impl<T: Default> FastArray<T> {
    /// ## Info
    /// creates a new [`FastArray`] of the given len and fills it with the [`Default`] value of the type T.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::FastArray;
    /// 
    /// let fast_array = FastArray::<usize>::new_default(2);
    /// 
    /// assert_eq!(fast_array.to_string(), "[0, 0]");
    /// ```
    /// 
    /// ## Panics
    /// panics if len == 0.
    #[inline]
    pub fn new_default(len: usize) -> FastArray<T> {
        assert!(len != 0, "len cannot be 0!");

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        unsafe {
            for i in 0..len {
                raw_ptr.add(i).write(T::default());
            };
        };

        FastArray {
            pointer: raw_ptr,
            size: len,
        }
    }

    #[inline]
    /// ## Info 
    /// same as [`FastArray::new_default`], just doesn't check for `len != 0` for performance reasons.
    /// obviously, if the len *is* 0, it's undefined behavior. 
    pub unsafe fn new_default_unchecked(len: usize) -> FastArray<T> {
        // assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32).expect("failed to create layout");

        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        unsafe {
            for i in 0..len {
                raw_ptr.add(i).write(T::default());
            };
        };

        FastArray {
            pointer: raw_ptr,
            size: len,
        }
    }
}

impl<T: Clone> FastArray<T> {
    /// ## Info
    /// creates a new [`FastArray`] of the given len and fills it with the given fill_value.
    /// 
    /// ## Example 
    /// ```
    /// use fast_array::FastArray;
    ///
    /// let fast_arr = FastArray::new(3, 5);
    /// 
    /// assert_eq!(fast_arr.to_string(), "[5, 5, 5]");
    /// ```
    /// 
    /// ## Panics
    /// if len == 0.
    #[inline(always)]
    pub fn new(len: usize, fill_value: T) -> FastArray<T> {
        assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32)
            .expect("Failed to create layout");


        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        unsafe {
            for i in 0..len {
                raw_ptr.add(i).write(fill_value.clone());
            };
        };

        FastArray {
            pointer: raw_ptr,
            size: len,
        }
    }

    #[inline(always)]
    /// ## Info
    /// same functionality as [`FastArray::new`], just skips the `len != 0` check for performance reasons.
    /// if `len == 0`, using this method is undefined behavior.
    pub unsafe fn new_unchecked(len: usize, fill_value: T) -> FastArray<T> {
        // assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32)
            .expect("Failed to create layout");


        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        unsafe {
            for i in 0..len {
                raw_ptr.add(i).write(fill_value.clone());
            };
        };

        FastArray {
            pointer: raw_ptr,
            size: len,
        }
    }
}

impl<T> FastArray<T> {
    /// ## Info
    /// returns the length of the [`FastArray`].
    /// 
    /// ## Example 
    /// ```
    /// use fast_array::fast_arr;
    /// 
    /// let fast_arr = fast_arr!(1,2,3,4,5);
    /// 
    /// assert_eq!(fast_arr.len(), 5);
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size
    }

    /// ## Info
    /// creates a new [`FastIterator`] with references to the values of the original [`FastArray`]
    /// 
    /// ## Example
    /// ```
    /// use fast_array::FastArray;
    /// let fast_arr = FastArray::new(5, 3);
    /// 
    /// let iter = fast_arr.iter();
    /// ```
    pub fn iter(&self) -> FastIterator<&T> {
        let mut index = 0;

        let func = || {
            index+=1;

            &self[index-1]
        };

        FastIterator::<&T>::new_func(self.size, func)
    }

    /// ## Info
    /// creates a new [`FastIterator`] that holds mutable references to the elements of the [`FastArray`].
    /// ```
    /// use fast_array::FastArray;
    /// 
    /// let mut fast_arr = FastArray::new(5, 3);
    /// 
    /// let iter_mut = fast_arr.iter_mut();
    /// ```
    pub fn iter_mut(&mut self) -> FastIterator<&mut T> {
        let mut index = 0;
        let size = self.size;
        
        let func = move || {
            
            let element = unsafe { &mut *self.pointer.add(index) };
            index += 1;
            element
        };
    
        FastIterator::<&mut T>::new_func(size, func)
    }

    /// ## Info
    /// creates a new [`FastArray`] based on a function or closure.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::FastArray;
    /// 
    /// let mut val = 0;
    /// let func = || {
    ///     val+=1;
    ///     val
    /// };
    /// 
    /// let fast_arr = FastArray::new_func(5, func);
    /// assert_eq!(fast_arr.to_string(), "[1, 2, 3, 4, 5]");
    /// ```
    /// 
    /// ## Panics
    /// if len == 0.
    pub fn new_func<F>(len: usize, mut func: F) -> FastArray<T> 
    where 
        F: FnMut () -> T,
    {
        assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32)
            .expect("Failed to create layout");


        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        unsafe {
            for i in 0..len {
                raw_ptr.add(i).write(func());
            };
        };

        FastArray {
            pointer: raw_ptr,
            size: len,
        }
    }

    /// ## Info
    /// same functionality as [`FastArray::new_func`], just skips the `len != 0` check for performance reasons.
    /// if `len == 0`, using this function is undefined behavior
    pub unsafe fn new_func_unchecked<F>(len: usize, mut func: F) -> FastArray<T> 
    where 
        F: FnMut () -> T,
    {
        // assert!(len != 0);

        // let layout = Layout::array::<T>(len).expect("failed to create layout");
        let layout = Layout::from_size_align(len * std::mem::size_of::<T>(), 32)
            .expect("Failed to create layout");


        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        unsafe {
            for i in 0..len {
                raw_ptr.add(i).write(func());
            };
        };

        FastArray {
            pointer: raw_ptr,
            size: len,
        }
    }

    #[inline(always)]
    /// ## Info
    /// makes a new empty [`FastArray`].
    /// 
    /// ## Warning
    /// every element of this array will be a null pointer when read if you don't write it first, so use with caution.
    pub unsafe fn new_empty(len: usize) -> FastArray<T> {
        assert!(len != 0);

        let layout = Layout::array::<T>(len).expect("failed to create layout");

        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        FastArray {
            pointer: raw_ptr,
            size: len,
        }
    }


    /// ## Info
    /// this method has the same functionality as [`FastArray::new_empty`], but skips the `len != 0` check for performance reasons.
    /// 
    /// If `len == 0`, using this method becomes undefined behavior
    pub unsafe fn new_empty_unchecked(len: usize) -> FastArray<T> {
        // assert!(len != 0);

        let layout = Layout::array::<T>(len).expect("failed to create layout");

        let raw_ptr = unsafe {
            alloc(layout) as *mut T
        };

        if raw_ptr.is_null() { panic!("Memory alloc failed.") };

        FastArray {
            pointer: raw_ptr,
            size: len,
        }
    }

    /// ## Info
    /// swaps two values based on the given indexes.
    /// 
    /// ## Warning
    /// this function *technically* doesn't need to borrow self mutably, but this is done to ensure the enforcement of the borrow checker.
    /// if you don't want the function to have this safety, use [`FastArray::swap_unsafe()`]
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    /// 
    /// let mut fast_arr = fast_arr!(1,2,3,4,5);
    /// fast_arr.swap(1, 4);
    /// 
    /// assert_eq!(fast_arr.to_string(), "[1, 5, 3, 4, 2]")
    /// ```
    /// 
    /// ## Panics
    /// if either of the indexes goes out of the bounds of the array.
    pub fn swap(&mut self, index1: usize, index2: usize) {
        assert!(self.size>index1 && self.size>index2);

        let pointer1 = self.get_mut_pointer(index1);
        let pointer2 = self.get_mut_pointer(index2);

        unsafe {
            ptr::swap(pointer1, pointer2);
        }
    }


    /// ## Info
    /// this method has the same functionality as [`FastArray::swap`], but skips the `self.len() > index1 && self.len() > index2` check for performance reasons.
    /// 
    /// If `self.len() > index1 && self.len() > index2` isn't respected, using this method becomes undefined behavior
    pub unsafe fn swap_unchecked(&mut self, index1: usize, index2: usize) {
        // assert!(self.size>index1 && self.size>index2);

        let pointer1 = self.get_mut_pointer_unchecked(index1);
        let pointer2 = self.get_mut_pointer_unchecked(index2);

        unsafe {
            ptr::swap(pointer1, pointer2);
        }
    }

    /// ## Warning
    /// you're probably searching for [`FastArray::swap`].
    /// 
    /// **this method is unsafe and the borrow checking rules get BROKEN here.**
    /// 
    /// only use this method if absolutely necessary.
    /// 
    /// ## Info
    /// does the exact same thing as [`FastArray::swap()`] does, just doesn't mutably borrow the array to do so.
    /// Instead, it swaps the underlying raw pointers using [`ptr::swap`].
    /// 
    /// ## Unsafe
    /// it's up to the user to guarantee the absence of undefined behavior when using this method.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    /// 
    /// let fast_arr = fast_arr!(1,2,3,4,5);
    /// unsafe { 
    ///     fast_arr.swap_unsafe(1, 4);
    /// }
    /// 
    /// assert_eq!(fast_arr.to_string(), "[1, 5, 3, 4, 2]")
    /// ``` 
    /// 
    /// ## Panics
    /// if either one of the indexes is out of bounds of the array.
    pub unsafe fn swap_unsafe(&self, index1: usize, index2: usize) {
        assert!(self.size>index1 && self.size>index2);

        let pointer1 = self.get_mut_pointer_unsafe(index1);
        let pointer2 = self.get_mut_pointer_unsafe(index2);

        unsafe {
            ptr::swap(pointer1, pointer2);
        }
    }

    /// ## Info
    /// this method has the same functionality as [`FastArray::swap_unsafe`], but skips the `self.len() > index1 && self.len() > index2` check for performance reasons.
    /// 
    /// If `self.len() > index1 && self.len() > index2` isn't respected, using this method becomes undefined behavior
    pub unsafe fn swap_unsafe_unchecked(&self, index1: usize, index2: usize) {
        // assert!(self.size>index1 && self.size>index2);

        let pointer1 = self.get_mut_pointer_unsafe_unchecked(index1);
        let pointer2 = self.get_mut_pointer_unsafe_unchecked(index2);

        unsafe {
            ptr::swap(pointer1, pointer2);
        }
    }

    /// ## Info
    /// returns the raw pointer to the specified index.
    /// 
    /// ## Warning
    /// this method is by itself safe, but it's up to the user to use the pointer correctly and safely.
    /// 
    /// ## Panics
    /// if index is out of bounds of the array.
    pub fn get_pointer(&self, index: usize) -> *const T {
    assert!(self.size>index);
        
        let pointer = unsafe { self.pointer.add(index) };
        pointer
    }

    /// ## Info
    /// this method has the same functionality as [`FastArray::get_pointer`], but skips the `self.len() > index` check for performance reasons.
    /// 
    /// If `self.len() > index` isn't respected, using this method becomes undefined behavior
    pub fn get_pointer_unchecked(&self, index: usize) -> *const T {
    // assert!(self.size>index);
        
        let pointer = unsafe { self.pointer.add(index) };
        pointer
    }

    /// ## Info
    /// returns the raw mutable pointer to the specified index.
    /// 
    /// It's not *technically* required to borrow self as mutable to do this, but this method does to enforce borrow checking rules.
    /// If you don't want the method to do so, use [`FastArray::get_mut_pointer_unsafe`]
    /// 
    /// ## Warning
    /// this method is by itself safe, but it's up to the user to use the pointer correctly and safely.
    /// 
    /// ## Panics
    /// if index is out of bounds of the array.
    pub fn get_mut_pointer(&mut self, index: usize) -> *mut T {
        assert!(self.size>index);
        
        let pointer = unsafe { self.pointer.add(index) };
        pointer
    }

    /// ## Info
    /// this method has the same functionality as [`FastArray::get_mut_pointer`], but skips the `self.len() > index` check for performance reasons.
    /// 
    /// If `self.len() > index` isn't respected, using this method becomes undefined behavior
    pub fn get_mut_pointer_unchecked(&self, index: usize) -> *mut T {
        // assert!(self.size>index);
            
            let pointer = unsafe { self.pointer.add(index) };
            pointer
        }

    /// ## Warning
    /// you're probably looking for [`FastArray::get_mut_pointer`].
    /// 
    /// **this method is unsafe and the borrow checking rules get BROKEN here.**
    ///
    /// it is not recommended to use this method if not strictly necessary.
    /// 
    /// ## Info
    /// returns the raw mutable pointer to the specified index, without borrowing self as mutable.
    /// 
    /// ## Panics
    /// if index is out of bounds of the array.
    pub unsafe fn get_mut_pointer_unsafe(&self, index: usize) -> *mut T {
        assert!(self.size>index);
        
        let pointer = unsafe { self.pointer.add(index) };
        pointer
    }

    /// ## Info
    /// this method has the same functionality as [`FastArray::get_mut_pointer_unsafe`], but skips the `self.len() > index` check for performance reasons.
    /// 
    /// If `self.len() <= index`, using this method becomes undefined behavior
    pub unsafe fn get_mut_pointer_unsafe_unchecked(&self, index: usize) -> *mut T {
        // assert!(self.size>index);
        
        let pointer = unsafe { self.pointer.add(index) };
        pointer
    }
}

impl FastArray<u8> {
    #[inline(always)]
    /// ## Info
    /// reads a file into the [`FastArray`] as bytes.
    pub fn read_file(&mut self, path: impl AsRef<str>) -> std::io::Result<()> {
        let file = File::open(path.as_ref())?;

        let mut buf_read = BufReader::new(file);
        buf_read.read(self.as_mut()).unwrap();
        Ok(())
    }

    /// ## Info
    /// creates a new [`FastArray`] containing the bytes of the file at the given path.
    pub fn new_read_file(path: impl AsRef<str>) -> std::io::Result<FastArray<u8>> {
        let file = File::open(path.as_ref())?;
        let size = file.metadata()?.size() as usize;
        let mut reader = BufReader::new(file);
        let mut fast_arr = unsafe { FastArray::new_empty(size) };
        reader.read(fast_arr.as_mut())?;
        Ok(fast_arr)
    }
}


impl<T> Index<usize> for FastArray<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(!(index>= self.size));

        unsafe { &*self.pointer.add(index) }
    }
}

impl<T> IndexMut<usize> for FastArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(!(index>= self.size));

        unsafe { &mut *self.pointer.add(index) }
    }
}

impl<T: Display> Display for FastArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        
        let mut first = true;
        for item in self.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
       
        write!(f, "]")
    }
}

impl<T> Drop for FastArray<T> {
    fn drop(&mut self) {
        if !self.pointer.is_null() {
            for i in 0..self.size {
                unsafe { ptr::drop_in_place(self.pointer.add(i)) };
            }

            // let layout = Layout::array::<T>(self.size).expect("Failed to create layout");
            let layout = Layout::from_size_align(self.size * std::mem::size_of::<T>(), 32).expect("failed to create layout");
            unsafe { dealloc(self.pointer as *mut u8, layout) };
        }
    }
}

impl<T: PartialEq> PartialEq for FastArray<T> {
    fn eq(&self, other: &Self) -> bool {
        let len = {
            let len = self.len();

            if len != other.len() {
                return false
            } else {
                len
            }
        };

        for i in 0..len {
            if self[i] != other[i] {
                return false
            }
        }

        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T: Eq> Eq for FastArray<T> {}

// #[test]
pub fn vec_iter() {
    let vec = (0..1600000).collect::<Vec<_>>();
    let mod_vec: Vec<_> = vec.into_iter().map(|x| x + 1).collect();
    drop(mod_vec);
}

// // #[test]
// pub fn fast_arr_iter() {
//     let fast_arr: FastArray<i32> = (0..1600000).into();
//     let iter: FastArray<i32> = fast_arr.as_fast_iterator().map(|x| x+1).as_fast_array();
//     drop(iter);
// }

// // #[test]
// pub fn fast_arr_simd() {
//     let mut fast_arr: FastArray<i32> = (0..1600000).into();
//     fast_arr.simd_add(1);
//     drop(fast_arr);
// }

// target/debug/deps/fast_array-53812f436eb8f409
use std::cmp::Ordering;

use crate::{fast_array::fast_array::FastArray, fast_iterator::fast_iterator::FastIterator};
// use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Serialize};

impl<T> FastArray<T> {
    /// ## Info
    /// gets an element at a given index.
    /// if the index is out of bounds, the function will return None.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    ///
    /// let array = fast_arr!(1,2,3,4,5);
    /// 
    /// assert_eq!(array.get(1), Some(&2));
    /// assert_eq!(array.get(100), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            None
        } else {
            Some(&self[index])
        }
    }

    #[inline(always)]
    /// ## Info
    /// turns a [`FastArray`] into a [`FastIterator`].
    /// this function has almost 0 overhead.
    /// 
    /// ## Example 
    /// ```
    /// use fast_array::fast_arr;
    ///
    /// let array = fast_arr!(1,2,3,4,5);
    /// 
    /// let iterator = array.into_fast_iterator();
    /// ```
    pub fn into_fast_iterator(mut self) -> FastIterator<T> {
        let pointer = self.pointer;
        self.pointer = std::ptr::null_mut(); // Invalidate the pointer
        
        FastIterator {
            pointer,
            len: self.size,
            current_index: (0, 0),
        }
    }
}

impl<T: Ord> FastArray<T> {
    /// ## Info
    /// uses the quicksort sorting algorithm to sort the array.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    ///
    /// let mut array = fast_arr!(3,5,1,4,2);
    /// array.sort();
    /// assert_eq!(array.to_string(), fast_arr!(1,2,3,4,5).to_string());
    /// ```
    pub fn sort(&mut self) {
        quicksort(self);
    }
}

impl<T> FastArray<T> {
    /// ## Info
    /// Sorts the array by a given function or closure.
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    ///
    /// let mut array = fast_arr!(3,5,1,4,2);
    /// array.sort_by(|a, b| b.cmp(&a)); // sort the array in reverse order
    /// assert_eq!(array.to_string(), fast_arr!(5,4,3,2,1).to_string())
    /// ```
    pub fn sort_by<F: FnMut (&T, &T) -> Ordering>(&mut self, sort_func: F) {
        quicksort_custom_sort(self, sort_func);
    }
}

pub(crate) fn quicksort<T: Ord>(arr: &mut FastArray<T>) {
    _quicksort(arr, 0, (arr.size - 1) as isize, &mut |a,b| a.cmp(&b));
}

pub(crate) fn quicksort_custom_sort<T, F: FnMut (&T, &T) -> Ordering>(arr: &mut FastArray<T>, mut sort_func: F) {
    _quicksort(arr, 0, (arr.size - 1) as isize, &mut sort_func);
}

fn _quicksort<T, F: FnMut (&T, &T) -> Ordering>(arr: &mut FastArray<T>, left: isize, right: isize, sort_func: &mut F) {
    if left <= right {
        let partition_idx = partition(arr, 0, right, sort_func);

        _quicksort(arr, left, partition_idx - 1, sort_func);
        _quicksort(arr, partition_idx + 1, right, sort_func);
    }
}

fn partition<T, F: FnMut (&T, &T) -> Ordering>(arr: &mut FastArray<T>, left: isize, right: isize,  sort_func: &mut F) -> isize {
    let pivot = right;
    let mut i: isize = left as isize - 1;

    for j in left..=right - 1 {
        match sort_func(&arr[j as usize], &arr[pivot as usize]) {
            Ordering::Less | Ordering::Equal => {
                i += 1;
                arr.swap(i as usize, j as usize);
            }

            Ordering::Greater => ()
        }
    }

    arr.swap((i + 1) as usize, pivot as usize);

    i + 1
}

impl<T: ToString> FastArray<T> {

    /// ## Info
    /// concatenates all the elements of the array into a [`String`] without any separators.
    /// if you need to concatenate them with separators, use `join()`
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    /// 
    /// let array = fast_arr!("Hello", ", ", "World", "!");
    /// 
    /// assert_eq!("Hello, World!", array.concat())
    /// ```
    pub fn concat(self) -> String {
        let mut string = String::new();
        for x in self {
            string.push_str(&x.to_string());
        }

        string
    }

    /// ## Info
    /// joins the elements of the array into a [`String`] with a separator in between them.
    /// if you need to join them without any separators, use `concat()`
    /// 
    /// ## Example
    /// ```
    /// use fast_array::fast_arr;
    /// 
    /// let array = fast_arr!("Hello", "World", "!");
    /// 
    /// assert_eq!("Hello World !", array.join(" "));
    /// ```
    pub fn join(self, sep: impl AsRef<str>) -> String {
        let sep = sep.as_ref();
        let iter = self.into_fast_iterator();
        let mut first = false;
        let new_self = iter
            .map(|item| {
                if !first {
                    first = true;
                    item.to_string()
                } else {
                    format!("{}{}", sep, item.to_string())
                }
            })
            .collect::<FastArray<_>>();
        new_self.concat()
    }
}

impl<T> IntoIterator for FastArray<T> {
    type Item = T;

    type IntoIter = FastIterator<Self::Item>;

    #[doc(alias = "into_fast_iterator")]
    fn into_iter(self) -> Self::IntoIter {
        self.into_fast_iterator()
    }
}

impl AsMut<[u8]> for FastArray<u8> {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.pointer, self.size) }
    }
}

impl<U> FromIterator<U> for FastArray<U> {
    fn from_iter<T: IntoIterator<Item = U>>(iter: T) -> Self {
        let iterator = iter.into_iter();

        // Collect the elements into a temporary Vec to determine the size
        let items: Vec<U> = iterator.collect();
        let size = items.len();

        // Create the FastArray with the exact size
        let mut array = unsafe { FastArray::new_empty(size) };

        // Move the items from the Vec into the FastArray
        for (index, item) in items.into_iter().enumerate() {
            array[index] = item;
        }

        array
    }
}

/// ## Info
/// trait implemented for types that can be turned into a [`FastArray`] with little to no overhead.
pub trait IntoFastArray<T> {
    fn into_fast_array(self) -> FastArray<T>;
}

impl<T, I: ExactSizeIterator<Item = T>, >  IntoFastArray<T> for I {
    #[inline(always)]
    /// ## Info
    /// turns the iterator into a [`FastArray`] with little overhead.
    fn into_fast_array(mut self) -> FastArray<T> {
        let size = self.len();
        let func = |_| self.next().unwrap();

        FastArray::new_func(size, func)
    }
}

impl<T> From<Vec<T>> for FastArray<T> {
    fn from(value: Vec<T>) -> Self {
        let len = value.len();
        let mut iter = value.into_iter();
        let func = |_| iter.next().unwrap();
        Self::new_func(len, func)
    }
}

#[cfg(feature = "serde")]
use std::marker::PhantomData;
#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for FastArray<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_seq(Some(self.size))?;
        for item in self.iter() {
            serde::ser::SerializeSeq::serialize_element(&mut state, &item)?;
        }
        serde::ser::SerializeSeq::end(state)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for FastArray<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct FastArrayVisitor<T> {
            phant: PhantomData<T>
        }

        impl<'de, U: serde::Deserialize<'de>> serde::de::Visitor<'de> for FastArrayVisitor<U> {
            type Value = FastArray<U>;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expecting an array")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>, 
            {
                let _len = seq.size_hint().unwrap_or({
                    let mut vec = Vec::new();

                    while let Some(item) = seq.next_element()? {
                        vec.push(item);
                    }

                    return Ok(vec.into())
                });

                if _len == 1 {
                    let mut vec = Vec::new();

                    while let Some(item) = seq.next_element()? {
                        vec.push(item);
                    }

                    return Ok(vec.into())
                }

                let iter = |_| seq.next_element::<U>();

                FastArray::new_func(_len, iter);
            }
        }

        deserializer.deserialize_seq(FastArrayVisitor::<T> { phant: PhantomData::<T>})
    }
}

impl<T: Clone> From<&mut [T]> for FastArray<T> {
    fn from(value: &mut [T]) -> Self {
        let len = value.len();
        let mut iter = value.iter();
        let func = |_| iter.next().unwrap().clone();

        FastArray::new_func(len, func)
    }
}

impl<T: Clone> From<&[T]> for FastArray<T> {
    fn from(value: &[T]) -> Self {
        let len = value.len();
        let mut iter = value.iter();
        let func = |_| iter.next().unwrap().clone();

        FastArray::new_func(len, func)
    }
}

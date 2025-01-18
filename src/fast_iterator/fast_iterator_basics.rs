use crate::fast_array::fast_array::FastArray;
use super::fast_iterator::FastIterator;

impl<T> FastIterator<T> {
    pub fn as_fast_array(mut self) -> FastArray<T> {
        let size = self.len;
        let pointer = self.pointer;
        self.pointer = std::ptr::null_mut();
        FastArray { pointer, size: size }
    }
}

impl<U> FromIterator<U> for FastIterator<U> {
    fn from_iter<X: IntoIterator<Item = U>>(iter: X) -> Self {
        let mut iterator = iter.into_iter();
        // let mem_size = std::mem::size_of_val(&iterator);
        let hint_size = iterator.size_hint();
        if let (lower, Some(upper)) = hint_size {
            if lower != upper {
                panic!("the iterator doesn't have an exact known size.");
            }

            let func = || iterator.next().unwrap();
            FastIterator::new_func(lower, func)
        } else {
            panic!("the iterator doesn't have an exact known size.")
        }
    }
}

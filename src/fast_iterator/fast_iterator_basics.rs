use super::fast_iterator::FastIterator;
use crate::fast_array::fast_array::FastArray;

impl<T> FastIterator<T> {
    pub fn into_fast_array(mut self) -> FastArray<T> {
        let size = (self.len-self.current_index.0) - self.current_index.1;
        let pointer = self.pointer;

        if self.current_index.0 == 0 && self.current_index.1 == 0 {
            self.pointer = std::ptr::null_mut();
            FastArray {
                pointer,
                size: size,
            }
        } else {
            let func = |_| {
                self.next().unwrap()
            };
            let fast_arr = FastArray::new_func(size, func);
            drop(self);
            fast_arr
        }
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

#![cfg(feature = "rayon")]

use rayon::{iter::{plumbing::{bridge, bridge_unindexed, Consumer, Producer, ProducerCallback, UnindexedProducer}, IndexedParallelIterator, ParallelIterator}, prelude::IntoParallelIterator};

use crate::FastIterator;

impl<T: Send + Sync> ParallelIterator for FastIterator<T> {
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> 
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len)
    }
}

// impl<T> UnindexedProducer for FastIterator<T> {
//     type Item = T;

//     fn split(mut self) -> (Self, Option<Self>) {
//         if self.len() != 1 {
//             let first_len = ((self.len() as f32) / 2.0).ceil() as usize;
//             let second_len = ((self.len() as f32) / 2.0).floor() as usize;
            
//             let mut func = || {
//                 self.next().unwrap()
//             };

//             let first = unsafe { Self::new_func_unchecked(first_len, &mut func) };
//             let second = unsafe { Self::new_func_unchecked(second_len, func) };

//             (first, Some(second))
//         } else {
//             (self, None)
//         }
        
//     }

//     fn fold_with<F>(self, folder: F) -> F
//     where
//         F: rayon::iter::plumbing::Folder<Self::Item> 
//     {
//         folder.consume_iter(self)
//     }
// }

impl<T: Send + Sync> IndexedParallelIterator for FastIterator<T> {
    fn with_producer<CB: ProducerCallback<Self::Item>>(
        self,
        callback: CB,
    ) -> CB::Output {
        callback.callback(self)
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.len
    }
}

use std::iter::DoubleEndedIterator;

impl<T: Send + Sync> Producer for FastIterator<T> {
    type Item = T;

    type IntoIter = FastIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self
    }

    fn split_at(mut self, index: usize) -> (Self, Self) {
        let len = self.len;
        
        let mut func = || {
            self.next().unwrap()
        };

        let first = unsafe { Self::new_func_unchecked(index, &mut func) };
        let second = unsafe {
            Self::new_func_unchecked(len - index, &mut func)
        };

        (first, second)
    }
}


use super::Matrix;
use core::iter::StepBy;
use core::slice::{self, Chunks, ChunksMut};
use num::Num;

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type IntoIter = Chunks<'a, T>;
    type Item = &'a [T];
    fn into_iter(self) -> Self::IntoIter {
        self.nums.chunks(self.n)
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type IntoIter = ChunksMut<'a, T>;
    type Item = &'a mut [T];
    fn into_iter(self) -> Self::IntoIter {
        self.nums.chunks_mut(self.n)
    }
}


pub struct Cols<'a, T> {
    matrix: &'a Matrix<T>,
    idx: usize,
}

impl<'a, T> Cols<'a, T> {
    pub fn new(matrix: &'a Matrix<T>) -> Self {
        Self { matrix, idx: 0 }
    }
}

impl<'a, T> Iterator for Cols<'a, T>
where
    T: Num + Clone + Copy,
{
    type Item = StepBy<slice::Iter<'a, T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.matrix.width() {
            None
        } else {
            let idx = self.idx;
            self.idx += 1;
            Some(
                self.matrix.nums[idx..]
                    .iter()
                    .step_by(self.matrix.width()),
            )
        }
    }
}



// pub struct ColsMut<'a, T> {
//     matrix: &'a mut Matrix<T>,
//     idx: usize,
// }
//
// impl<'a, T> ColsMut<'a, T> {
//     pub fn new(matrix: &'a mut Matrix<T>) -> Self {
//         Self { matrix, idx: 0 }
//     }
// }
//
// impl<'a, T> Iterator for ColsMut<'a, T>
// where
//     T: Num + Clone + Copy + std::fmt::Debug,
// {
//     type Item = StepBy<slice::IterMut<'a, T>>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx >= self.matrix.width() {
//             None
//         } else {
//             let idx = self.idx;
//             self.idx += 1;
//             Some(
//                 self.matrix.nums[idx..]
//                     .iter_mut()
//                     .step_by(self.matrix.width()),
//             )
//         }
//     }
// }

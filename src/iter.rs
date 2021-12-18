use super::{
    Matrix,
    num_trait::Prim

};

use std::iter::StepBy;
use std::slice::{self, Chunks, ChunksMut};

impl<'a, T> IntoIterator for &'a Matrix<T>
where
    T: Prim,
{
    type IntoIter = Chunks<'a, T>;
    type Item = &'a [T];
    fn into_iter(self) -> Self::IntoIter {
        self.nums.chunks(self.height())
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T>
where
    T: Prim,
{
    type IntoIter = ChunksMut<'a, T>;
    type Item = &'a mut [T];
    fn into_iter(self) -> Self::IntoIter {
        self.nums.chunks_mut(self.height())
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
    T: Prim,
{
    type Item = StepBy<slice::Iter<'a, T>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.matrix.width() {
            None
        } else {
            let idx = self.idx;
            self.idx += 1;
            Some(self.matrix.nums[idx..].iter().step_by(self.matrix.width()))
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
//     T: Num + Clone,
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

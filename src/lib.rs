use num::Num; // 0.4.0
use std::clone::Clone;
use std::slice::{Chunks, ChunksMut};

pub mod iter;
pub mod ops;

pub mod prelude {
    pub use super::iter;
    pub use super::ops::{self, Mul};
    pub use super::Matrix;
}




#[derive(Clone, Debug)]
pub struct Matrix<T> {
    nums: Box<[T]>,
    m: usize,
    n: usize,
}

impl<T> Matrix<T>
where
    T: Num + Clone + Copy,
{
    pub fn from_slice(nums: &[T], m: usize, n: usize) -> Self {
        assert_eq!(nums.len(), m * n);
        Self {
            nums: nums.to_vec().into_boxed_slice(),
            m,
            n,
        }
    }

    pub fn null(m: usize, n: usize) -> Self {
        Self {
            nums: vec![T::zero(); m * n].into_boxed_slice(),
            m,
            n,
        }
    }

    pub fn with(val: T, m: usize, n: usize) -> Self {
        Self {
            nums: vec![val; m * n].into_boxed_slice(),
            m,
            n,
        }
    }

    pub fn deter(&self) -> T {
        todo!()
    }

    pub fn height(&self) -> usize {
        self.n
    }

    pub fn width(&self) -> usize {
        self.m
    }

    pub fn iter(&self) -> Chunks<T> {
        self.nums.chunks(self.n)
    }

    pub fn iter_mut(&mut self) -> ChunksMut<T> {
        self.nums.chunks_mut(self.n)
    }

    pub fn cols(&self) -> iter::Cols<'_, T> {
        iter::Cols::new(self)
    }

    // pub fn cols_mut(&mut self) -> iter::RowsMut<'_, T> {
    //     iter::ColsMut::new(self)
    // }
}

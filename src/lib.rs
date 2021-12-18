use std::clone::Clone;
use std::slice::{Chunks, ChunksMut};

pub mod iter;
pub mod ops;
pub mod num_trait;
use num_trait::Prim;
/*
#[cfg(test)]
mod tests; 
*/
pub mod prelude {
    pub use super::iter;
    pub use super::Matrix;
    pub use super::num_trait::Prim;
}

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    nums: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T>
where
    T: Prim,
{
    // constructors/reuse

    pub fn from_arrays<const W: usize, const H: usize>(arrays: [[T; W]; H]) -> Self {
        Self {
            nums: arrays.into_iter().flatten().collect::<Vec<T>>().into_boxed_slice(),
            width: W,
            height: H,
        }
    }

    pub fn from_slice(nums: &[T], width: usize, height: usize) -> Self {
        assert_eq!(nums.len(), width * height);
        Self {
            nums: nums.to_vec().into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn from_val(val: T, width: usize, height: usize) -> Self {
        Self {
            nums: vec![val; width * height].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn from_val_with_buf(mut buf: Vec<T>, val: T, width: usize, height: usize) -> Self {
        buf.resize(width*height, val);
        Self {
            nums: buf.into_boxed_slice(),
            width,
            height,
        }
    }
    
    pub fn null(width: usize, height: usize) -> Self {
        Self {
            nums: vec![T::zero(); width * height].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn into_buf(self) -> Vec<T> {
        let mut buf = Vec::from(self.nums);
        buf.clear();
        buf
    }
    
    /// resizes a Vec<T> to have the same length
    /// as self and fills it with zeroes
    fn reserve_equal(&self, buf: &mut Vec<T>) {
        buf.resize(self.nums.len(), T::zero());
    }
    
    /// returns an identical matrix from a given buffer
    fn copy_to_buf(&self, mut buf: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(self.width()*self.height(), width*height);

        self.reserve_equal(&mut buf);
        let mut buf = buf.into_boxed_slice();
        buf.copy_from_slice(&self.nums);
        Self {
            nums: buf,
            width,
            height,
        }
    }


    // getters/properties
    
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
    
    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x <= self.width() && y <= self.height()
    }


    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        // even though x*w + y might be in self.nums,
        // it might still be out of bounds for the matrix
        // coordinates and return an unexpected value
        // or an error
        if self.in_bounds(x, y) {
            self.nums.get(x*self.width() + y).copied()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if self.in_bounds(x, y) {
            self.nums.get_mut(x*self.width() + y)
        } else {
            None
        }
    }
    
    fn get_buf_idx(&self, x: usize, y: usize) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some(x*self.width() + y)
        } else {
            None
        }
    }


    // iterators
     
    pub fn iter(&self) -> Chunks<T> {
        self.nums.chunks(self.width())
    }

    pub fn iter_mut(&mut self) -> ChunksMut<T> {
        self.nums.chunks_mut(self.width())
    }

    pub fn cols(&self) -> iter::Cols<'_, T> {
        iter::Cols::new(self)
    }

    pub fn as_slice(&self) -> &[T] {
        &self.nums
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.nums
    }
    
    /*
    pub fn cols_mut(&mut self) -> iter::RowsMut<'_, T> {
       iter::ColsMut::new(self)
    }
    */


    // operations

    // -> T

    pub fn determ_dispatch(&self) -> T {
        assert!(self.is_square());
        
        match self.width() {
            0 => T::one(),
            1 => self.nums[0],
            2 =>{
                let [a, b, c, d]: [T; 4] = self.nums.as_ref().try_into().unwrap();
                a*c - b*d
            }
            3 => self.determ_sarrus(),
            _ => self.determ_laplace_rec()
        } 
    }

    pub fn determ_sarrus(&self) -> T {
        assert_eq!((self.width(), self.height()), (3, 3));
        
        let mut deter = T::zero();
        
        for c in 0..=2 {
            let (mut prod_pri, mut prod_sec) = (T::one(), T::one());
            for i in 0..=2 {
                prod_pri *= self.get(i, (i+c) % 3).unwrap();
                prod_sec *= self.get(i, (i+i+c) % 3).unwrap();
            }
            deter += prod_pri;
            deter -= prod_sec;
        }
        deter
    }

    pub fn determ_laplace_rec(&self) -> T {
        todo!()
    }


    // -> Self

    pub fn sq_transpose_to(&self, buf: Vec<T>) -> Self {
        assert!(self.is_square());
        // only works for square matrices??
        let mut out = self.copy_to_buf(buf, self.height(), self.width());
        if self.height <= 1 {
            return out;
        }
        //  for mat 3x3 01 02, 12
        //  for mat 4x4 01 02 03, 12 13, 23
        // 
         
        for y in 0..=(self.height()-2) {
            for x in (y+1)..=(self.width()-1) {
                let (a, b) = (
                    self.get_buf_idx(y, x).unwrap(),
                    self.get_buf_idx(x, y).unwrap(),
                );
                println!("swapping [{}][{}]={} and [{}][{}]={}", y, x, a, x, y, b);
                out.nums.swap(a, b);
            }
        }
        out
    }

    pub fn transpose_to(&self, buf: Vec<T>) -> Self {
        if self.is_square() {
           self.sq_transpose_to(buf)
        } else {
            let mut out = Self::null(self.height(), self.width());
            for (x, line) in self.iter().enumerate() {
                for (y, elem) in line.iter().enumerate() {
                    *out.get_mut(y, x).unwrap() = *elem;
                }
            }
            out
        }

    }


    // apply for each

    pub fn apply<F>(&mut self, func: F)
    where
        F: Fn(T) -> T
    {
        self.nums.iter_mut().for_each(|num| *num = func(*num));
    }


    pub fn apply_to<F>(&self, func: F, buf: Vec<T>) -> Self
    where
        F: Fn(T) -> T 
    {
        let mut out = self.copy_to_buf(buf, self.width(), self.height());
        out.apply(func);
        out
    }
    


    //  operations
    //      return Matrix
    //          ~~op_to(&self, Vec)
    //          ~~op_apply(&mut self)
    //          ??op(&self) -> Self
    //      unary
    //          ~~det 1x1, 2x2
    //          ~~det sarrus
    //          det laplace(recursive? no alloc?)
    //          inv elem?? how
    //          inv adj
    //          tsp square
    //          ~~tsp general
    //      ~~scalar
    //          add, sub, mul, div
    //      
}



#[non_exhaustive]
pub enum CalcError {
    DiffDimensions,
    NotEqualRowCol,
}

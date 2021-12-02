use super::Matrix;
use num::Num;



impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        let start = idx * self.m;
        let end = start + self.m;
        &self.nums[start..end]
    }
}


pub trait Mul<T, Rhs> {
    fn mul(&self, rhs: Rhs) -> Result<Matrix<T>, CalcError>;

    fn mul_assign(&mut self, rhs: Rhs) -> Result<(), CalcError>;
}

impl<T> Mul<T, T> for Matrix<T>
where
    T: std::ops::Mul<T, Output = T> + Clone + Copy,
{
    fn mul(&self, rhs: T) -> Result<Self, CalcError> {
        let mut output = Vec::with_capacity(self.m * self.n);
        for x in self.nums.iter() {
            output.push(*x * rhs);
        }
        Ok(Self {
            nums: output.into_boxed_slice(),
            m: self.m,
            n: self.n,
        })
    }

    fn mul_assign(&mut self, rhs: T) -> Result<(), CalcError> {
        for num in self.nums.iter_mut() {
            *num = *num * rhs;
        }
        Ok(())
    }
}

impl<T> Mul<T, &Matrix<T>> for Matrix<T>
where
    T: Num + std::ops::Mul<T, Output = T> + Clone + Copy + std::iter::Sum,
{
    fn mul(&self, rhs: &Matrix<T>) -> Result<Self, CalcError> {
        if rhs.n != self.m {
            Err(CalcError::NotEqualRowCol)
        } else {
            let mut output = Vec::with_capacity(self.m * rhs.n);
            for line in self.iter() {
                for col in rhs.cols() {
                    output.push(line.iter().zip(col).map(|(x, y)| *x * *y).sum())
                }
            }
            Ok(Self {
                nums: output.into_boxed_slice(),
                m: self.m,
                n: rhs.n,
            })
        }
    }

    fn mul_assign(&mut self, rhs: &Matrix<T>) -> Result<(), CalcError> {
        *self = self.mul(rhs)?;
        Ok(())
    }
}

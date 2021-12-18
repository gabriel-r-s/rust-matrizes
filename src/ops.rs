use super::{
    Matrix,
    num_trait::Prim

};


impl<T> std::ops::Index<usize> for Matrix<T>
where
    T: Prim,
{
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        let start = idx * self.width();
        let end = start + self.width();
        &self.nums[start..end]
    }
}



/*
pub trait Mul<'a, Rhs>: Sized {
    fn mul(&'a self, rhs: &'a Rhs) -> Result<Self, CalcError>;

    fn mul_assign(&'a mut self, rhs: &'a Rhs) -> Result<(), CalcError>;
}

pub trait Mul<'a, Rhs>: 


impl<'a, T> Mul<'a, T> for Matrix<T>
where
    T: Num + Clone + 'a,
    &'a T: std::ops::Mul<&'a T, Output=T>,
{
    fn mul(&'a self, rhs: &'a T) -> Result<Self, CalcError> {
        Ok(Self::from_parts(
            self.nums.iter().map(|x| x * rhs).collect::<Vec<T>>().into_boxed_slice(),
            self.width(),
            self.height(),
        ))
    }

    fn mul_assign(&'a mut self, rhs: &'a T) -> Result<(), CalcError>
    where T: std::ops::MulAssign
    {
        Ok(
            self.nums.iter_mut().for_each(|x| {
                *x *= rhs
            })
        )
    }
}
*/

use num::{
    Num,
    Bounded,
    NumCast,
};

use std::clone::Clone;
use std::marker::Copy;
use std::cmp::PartialOrd;

use std::ops::{
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
};

pub trait Prim: 
    Num
    + Bounded
    + NumCast
    
    + PartialOrd
    + Clone
    + Copy
    
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{}


impl<P> Prim for P
where P: 
    Num
    + Bounded
    + NumCast
    
    + PartialOrd
    + Clone
    + Copy
    
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{}

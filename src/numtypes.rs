use rug::*;
use std::ops::{Add, Sub, Mul, Shl, Shr};
use std::cmp::min;
use std::fmt;

use crate::constants::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntExp {
    pub val: Integer,
    pub exp: i32,
}

// Borrowed-borrowed operations (zero clones needed) - &T + &T -> T
impl Add<&IntExp> for &IntExp {
    type Output = IntExp;
    fn add(self, other:&IntExp) -> IntExp {
        let smallest_exp = min(self.exp, other.exp);
        let self_shift = self.exp - smallest_exp;
        let other_shift = other.exp - smallest_exp;
        assert!(self_shift >= 0 && other_shift >= 0);
        let sum = (self.val.clone() << self_shift as u32) + (other.val.clone() << other_shift as u32);
        IntExp { val: sum, exp: smallest_exp }
    }
}

impl Sub<&IntExp> for &IntExp {
    type Output = IntExp;
    fn sub(self, other:&IntExp) -> IntExp {
        let smallest_exp = min(self.exp, other.exp);
        let self_shift = self.exp - smallest_exp;
        let other_shift = other.exp - smallest_exp;
        assert!(self_shift >= 0 && other_shift >= 0);
        let sum = (self.val.clone() << self_shift as u32) - (other.val.clone() << other_shift as u32);
        IntExp { val: sum, exp: smallest_exp }
    }
}

impl Mul<&IntExp> for &IntExp {
    type Output = IntExp;
    fn mul(self, other:&IntExp) -> IntExp {
        IntExp { val: self.val.clone() * other.val.clone(), exp: self.exp + other.exp }
    }
}

// Owned-owned operations (delegate to borrowed for consistency)
impl Add for IntExp {
    type Output = Self;
    fn add(self, other:Self) -> Self {
        (&self).add(&other)
    }
}

impl Sub for IntExp {
    type Output = Self;
    fn sub(self, other:Self) -> Self {
        (&self).sub(&other)
    }
}

impl Mul for IntExp {
    type Output = Self;
    fn mul(self, other:Self) -> Self {
        (&self).mul(&other)
    }
}

// Owned-borrowed operations (for flexibility)
impl Add<&IntExp> for IntExp {
    type Output = Self;
    fn add(self, other:&IntExp) -> Self {
        (&self).add(other)
    }
}

impl Sub<&IntExp> for IntExp {
    type Output = Self;
    fn sub(self, other:&IntExp) -> Self {
        (&self).sub(other)
    }
}

impl Mul<&IntExp> for IntExp {
    type Output = Self;
    fn mul(self, other:&IntExp) -> Self {
        (&self).mul(other)
    }
}

impl Shl<u32> for IntExp {
    type Output = IntExp;
    fn shl(self, rhs: u32) -> Self::Output {
        Self{ val: self.val, exp: self.exp + rhs as i32 }
    }
}

impl Shr<u32> for IntExp {
    type Output = IntExp;
    fn shr(self, rhs: u32) -> Self::Output {
        Self{ val: self.val, exp: self.exp - rhs as i32 }
    }
}

impl From<i32> for IntExp {
    fn from(value: i32) -> Self {
        Self{val:Integer::from(value), exp:0}
    }
}

impl fmt::Display for IntExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.val.significant_bits() > INTEXP_WARNING_SIZE {
            println!("WARNING: intexp passed warning size");
        }
        if self.exp >= 0 {
            write!(f, "{}", (self.val.clone()<<self.exp as u32).to_string())?;
            Ok(())
        } else {
            write!(f, "{}...", self.val.clone()>>(-self.exp as u32)).and_then(|_| {
                f.write_str(".")
            }).and_then(|_| {
                Ok(())
            })
        }
    }
}

impl IntExp {
    pub fn shift(self, exp: i32) -> IntExp {
        if exp >= 0 {
            self << exp as u32
        } else {
            self >> (-exp) as u32
        }
    }
    pub fn round(self, _bits: usize) -> IntExp {
        IntExp{val: self.val >> 1, exp: self.exp + 1}
    }
}

// Mandelbrotable trait with borrowed-borrowed operations (&T + &T -> T)
pub trait Mandelbrotable: 
    Clone 
    + Add<Output=Self> 
    + Sub<Output=Self> 
    + Mul<Output=Self>
    + for<'a> Add<&'a Self, Output=Self> 
    + for<'a> Sub<&'a Self, Output=Self> 
    + for<'a> Mul<&'a Self, Output=Self>
    + PartialOrd 
    + Sized 
{
    fn from_f64(v: f64) -> Self;
}

// Implement Ord for IntExp
impl PartialOrd for IntExp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntExp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare by value first (scaled to same exponent), then by exponent
        let common_exp = self.exp.max(other.exp);
        let self_scaled = if common_exp >= self.exp { 
            self.val.clone() << (common_exp - self.exp) as u32 
        } else { 
            self.val.clone() >> (self.exp - common_exp) as u32 
        };
        let other_scaled = if common_exp >= other.exp { 
            other.val.clone() << (common_exp - other.exp) as u32 
        } else { 
            other.val.clone() >> (other.exp - common_exp) as u32 
        };
        self_scaled.cmp(&other_scaled)
    }
}

impl From<f64> for IntExp {
    fn from(value: f64) -> Self {
        // Convert f64 to IntExp by parsing the integer part
        let int_part = value.trunc() as i64;
        IntExp { val: Integer::from(int_part), exp: 0 }
    }
}

impl Mandelbrotable for IntExp {
    fn from_f64(v: f64) -> Self {
        let int_part = v.trunc() as i64;
        IntExp { val: Integer::from(int_part), exp: 0 }
    }
}

// Implement Mandelbrotable for f32, f64, and rug::Float
impl Mandelbrotable for f32 {
    fn from_f64(v: f64) -> Self { v as f32 }
}

impl Mandelbrotable for f64 {
    fn from_f64(v: f64) -> Self { v }
}

impl Mandelbrotable for Float {
    fn from_f64(v: f64) -> Self { 
        Float::with_val_64(1024, v)
    }
}

pub trait Abs {
    fn abs(self) -> Self;
}

impl Abs for f32 {
    fn abs(self) -> Self { self.abs() }
}

impl Abs for f64 {
    fn abs(self) -> Self { self.abs() }
}

pub trait Gt: Copy {
    fn gt(self, a:Self) -> bool;
}

pub trait Finite {
    fn is_finite(self) -> bool;
}

impl Finite for f32 {
    fn is_finite(self) -> bool { self.is_finite() }
}

impl Finite for f64 {
    fn is_finite(self) -> bool { self.is_finite() }
}
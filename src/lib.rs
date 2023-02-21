use std::ops::{Add, Mul, Sub, AddAssign, MulAssign};
use num_traits::{One, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zn<const N: u128>
{
    value: u128
}

impl<const N:u128> Zn<N>
{
    pub fn new(value: u128) -> Self {
        Zn {
            value: value % N
        }
    }
}

impl <const N:u128> Add for Zn<N>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Zn::new(self.value + rhs.value)
    }
}

impl <const N:u128> Sub for Zn<N>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.value < rhs.value {
            Zn::new(N - (rhs.value-self.value))
        } else {
            Zn::new(self.value - rhs.value)
        }
    }
}

impl <const N:u128> Mul for Zn<N>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Zn::new(self.value * rhs.value)
    }
}
impl<const N:u128> One for Zn<N>
{
    fn one() -> Self {
        Zn::new(1)
    }
}
impl<const N:u128> Zero for Zn<N>
{
    fn zero() -> Self {
        Zn::new(0)
    }
    fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl<const N:u128> AddAssign for Zn<N>
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<const N:u128> MulAssign for Zn<N>
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<const N:u128> From<Zn<N>> for u128
{
    fn from(zn: Zn<N>) -> Self {
        zn.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::matrix;

    #[test]
    fn test_add() {
        assert_eq!(Zn::<2>::new(1) + Zn::<2>::new(1), Zn::<2>::new(0));
        assert_eq!(Zn::<2>::new(1) + Zn::<2>::new(2), Zn::<2>::new(1));
    }
    #[test]
    fn test_sub() {
        assert_eq!(Zn::<2>::new(0) - Zn::<2>::new(1), Zn::<2>::new(1));
        assert_eq!(Zn::<2>::new(1) - Zn::<2>::new(2), Zn::<2>::new(1));
    }
    #[test]
    fn test_mul() {
        assert_eq!(Zn::<2>::new(1) * Zn::<2>::new(1), Zn::<2>::new(1));
        assert_eq!(Zn::<2>::new(1) * Zn::<2>::new(2), Zn::<2>::new(0));
    }
    #[test]
    fn mat_mul() {
        let a = matrix![Zn::<5>::new(1), Zn::<5>::new(2); Zn::<5>::new(3), Zn::<5>::new(4)];
        let b = matrix![Zn::<5>::new(1), Zn::<5>::new(2); Zn::<5>::new(3), Zn::<5>::new(4)];
        assert_eq!(a*b, matrix![Zn::<5>::new(2), Zn::<5>::new(0); Zn::<5>::new(0), Zn::<5>::new(2)]);
    }
}
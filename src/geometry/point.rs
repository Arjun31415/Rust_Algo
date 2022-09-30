use std::i128;
use std::ops;
pub trait PointType:
    Sized
    + core::ops::Add<Self, Output = Self>
    + core::ops::Sub<Self, Output = Self>
    + core::ops::Mul<Self, Output = Self>
    + core::ops::Div<Self, Output = Self>
    + core::ops::AddAssign<Self>
    + core::ops::SubAssign<Self>
    + core::ops::MulAssign<Self>
    + core::ops::DivAssign<Self>
    + Copy
{
    fn zero() -> Self
    where
        Self: Sized;
}
// Inspired from https://github.com/rust-num/num-traits/blob/master/src/identities.rs
impl PointType for i32 {
    fn zero() -> i32 {
        0
    }
}
impl PointType for i64 {
    fn zero() -> i64 {
        0
    }
}
impl PointType for f32 {
    fn zero() -> f32 {
        0.0
    }
}
impl PointType for f64 {
    fn zero() -> f64 {
        0.0
    }
}
impl PointType for i128 {
    fn zero() -> i128 {
        0
    }
}
#[derive(Copy, Clone)]
struct Point<T: PointType> {
    pub x: T,
    pub y: T,
}
impl<T: PointType> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}
impl<T: PointType> std::default::Default for Point<T> {
    fn default() -> Self {
        Point::new(T::zero(), T::zero())
    }
}
impl<T: PointType + std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
/// Add two points as vectors
impl<T: PointType> ops::Add<Point<T>> for Point<T> {
    fn add(self, rhs: Point<T>) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
    type Output = Point<T>;
}
/// subtract two points as vectors
impl<T: PointType> ops::Sub<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
/// multiply a point with a scalar
impl<T: PointType> ops::Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}
/// divide a point with a scalar
impl<T: PointType> ops::Div<T> for Point<T> {
    type Output = Point<T>;
    fn div(self, rhs: T) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}
/// implement AddAssign for a point
impl<T: PointType> ops::AddAssign<Point<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Point<T>) {
        *self = *self + rhs
    }
}
/// implement SubAssign for a point
impl<T: PointType> ops::SubAssign<Point<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: Point<T>) {
        *self = *self - rhs
    }
}
/// implement MulAssign for a point with a scalar
impl<T: PointType> ops::MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}
/// implement DivAssign for a point with a scalar
impl<T: PointType> ops::DivAssign<T> for Point<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_point() {
        let p1: Point<i64> = Point::new(1, 0);
        println!("{}", p1);
    }
}

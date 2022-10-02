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
    + std::cmp::PartialEq
    + std::cmp::PartialOrd
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
impl PointType for i128 {
    fn zero() -> i128 {
        0
    }
}
#[derive(Copy, Clone, Debug)]
struct Point<T: PointType> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T: PointType> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point { x, y, z }
    }
}
impl<T: PointType> std::default::Default for Point<T> {
    fn default() -> Self {
        Point::new(T::zero(), T::zero(), T::zero())
    }
}
impl<T: PointType + std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{}, {})", self.x, self.y, self.z)
    }
}
/// Add two points as vectors
impl<T: PointType> ops::Add<Point<T>> for Point<T> {
    fn add(self, rhs: Point<T>) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
    type Output = Point<T>;
}
impl<'a, 'b, T: PointType> ops::Add<&'b Point<T>> for &'a Point<T> {
    type Output = Point<T>;

    fn add(self, other: &'b Point<T>) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
/// subtract two points as vectors
impl<T: PointType> ops::Sub<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl<'a, 'b, T: PointType> ops::Sub<&'b Point<T>> for &'a Point<T> {
    type Output = Point<T>;

    fn sub(self, other: &'b Point<T>) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// multiply a point with a scalar
impl<T: PointType> ops::Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
/// divide a point with a scalar
impl<T: PointType> ops::Div<T> for Point<T> {
    type Output = Point<T>;
    fn div(self, rhs: T) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs, self.z / rhs)
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
impl<T: PointType> std::cmp::PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}
// for efficiently sorting points in Convex hull
impl<T: PointType> std::cmp::PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        } else if self < other {
            return Some(std::cmp::Ordering::Less);
        } else {
            assert!(!(self < other));
            return Some(std::cmp::Ordering::Greater);
        }
    }
    fn lt(&self, rhs: &Self) -> bool {
        if self.x == rhs.x {
            if self.y == rhs.y {
                return self.z < rhs.z;
            }
            return self.y < rhs.y;
        }
        return self.x < rhs.x;
    }
}
fn dot<T: PointType>(a: &Point<T>, b: &Point<T>) -> T {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}
trait GeometryOperations<T: PointType> {
    fn ang(a: &Point<T>, b: &Point<T>, deg: bool) -> f64;
    fn signed_area_of_parallelogram(a: Point<T>, b: Point<T>, c: Point<T>) -> f64;
    fn area_of_triangle(a: Point<T>, b: Point<T>, c: Point<T>) -> f64 {
        return f64::abs(Self::signed_area_of_parallelogram(a, b, c) * 0.5);
    }
    fn direction(a: Point<T>, b: Point<T>, c: Point<T>) -> i8 {
        return num_traits::signum(Self::signed_area_of_parallelogram(a, b, c)) as i8;
    }
    fn area_of_polygon(a: Vec<Point<T>>) -> f64;
}
// macro to generate the ang function for various types
macro_rules! ang_gen {
    ($t:ty) => {
        fn ang(a: &Point<$t>, b: &Point<$t>, deg: bool) -> f64 {
            let a_rad =
                f64::acos(dot(a, b) as f64 / (f64::sqrt(dot(a, a) as f64 * dot(b, b) as f64)));
            if deg {
                return f64::to_degrees(a_rad);
            }
            return a_rad;
        }
    };
}
macro_rules! area_of_parallelogram_gen {
    ($t:ty) => {
        fn signed_area_of_parallelogram(a: Point<$t>, b: Point<$t>, c: Point<$t>) -> f64 {
            return (cross(b - a, c - b)).z as f64;
        }
    };
}
macro_rules! area_of_poly_gen {
    ($t:ty) => {
        fn area_of_polygon(a: Vec<Point<$t>>) -> f64 {
            let n = a.len();
            let mut area: f64 = 0.0;
            for i in 0..n {
                area -= ((a[(i + 1) % n].x - a[i].x) * (a[(i + 1) % n].y + a[i].y)) as f64;
            }
            area /= 2.0;
            return area.abs();
        }
    };
}
impl GeometryOperations<f32> for Point<f32> {
    ang_gen!(f32);
    area_of_parallelogram_gen!(f32);
    area_of_poly_gen!(f32);
}
impl GeometryOperations<i32> for Point<i32> {
    ang_gen!(i32);
    area_of_parallelogram_gen!(i32);
    area_of_poly_gen!(i32);
}
impl GeometryOperations<i64> for Point<i64> {
    ang_gen!(i64);
    area_of_parallelogram_gen!(i64);
    area_of_poly_gen!(i64);
}
impl GeometryOperations<i128> for Point<i128> {
    ang_gen!(i128);
    area_of_parallelogram_gen!(i128);
    area_of_poly_gen!(i128);
}
fn cross<T: PointType>(a: Point<T>, b: Point<T>) -> Point<T> {
    let x0: T = a.x;
    let y0: T = a.y;
    let z0: T = a.z;
    let x1: T = b.x;
    let y1: T = b.y;
    let z1: T = b.z;
    return Point::new(y0 * z1 - z0 * y1, z0 * x1 - x0 * z1, x0 * y1 - y0 * x1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_point_operations() {
        let x1 = 20;
        let x2 = 45;
        let y1 = 12;
        let y2 = -100;
        let z1 = 30;
        let z2 = -55;
        let p1: Point<i64> = Point::new(x1, y1, z1);
        let p2: Point<i64> = Point::new(x2, y2, z2);
        assert_eq!(p1 + p2, Point::new(x1 + x2, y1 + y2, z1 + z2));
        assert_eq!(p1 - p2, Point::new(x1 - x2, y1 - y2, z1 - z2));
        assert_eq!(
            p1 * 2 - p2 * 3,
            Point::new(2 * x1 - 3 * x2, 2 * y1 - 3 * y2, 2 * z1 - 3 * z2)
        );
        assert_eq!(p1 / 2, Point::new(x1 / 2, y1 / 2, z1 / 2));
        assert_eq!(
            p1 / 2 - p2 / 3,
            Point::new(x1 / 2 - x2 / 3, y1 / 2 - y2 / 3, z1 / 2 - z2 / 3)
        );
        assert_eq!(dot(&p1, &p2), x1 * x2 + y1 * y2 + z1 * z2);
        assert_eq!(
            cross(p1, p2),
            Point::new(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
        )
    }
}

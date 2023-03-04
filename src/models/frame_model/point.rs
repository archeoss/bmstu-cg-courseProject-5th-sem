// use cgmath::{num_traits::Float, BaseFloat, Matrix4, toPoint3, Transform};
use nalgebra::{
    ComplexField, Const, Matrix4, Matrix4x1, Point3, RealField, Scalar, Vector3, Vector4,
};
use std::{
    hash::Hash,
    ops::{self, Neg},
};

use crate::macros::*;
#[derive(Copy, PartialEq, Clone, Debug, Default)]
pub struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl Hash for Point<f64> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.x.trunc() as i64).hash(state);
        (self.y.trunc() as i64).hash(state);
        (self.z.trunc() as i64).hash(state);
    }
}

impl<T: Copy + Scalar + RealField + std::ops::MulAssign + std::ops::AddAssign> Point<T> {
    #[must_use]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    getter_setter!(x: T, y: T, z: T);

    pub const fn xyz(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    #[must_use]
    pub const fn position(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    pub fn set_position(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn scalar_mul(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn lerp(&self, other: &Self, t: T) -> Self {
        let x = self.x + (other.x - self.x) * t;
        let y = self.y + (other.y - self.y) * t;
        let z = self.z + (other.z - self.z) * t;
        Self { x, y, z }
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn len(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    //
    // pub(crate) fn transform_first(&self, transform: &Matrix4<T>) -> Self
    // {
    //     let mut point = Vector3::<T>::new(self.x, self.y, self.z);
    //
    //     point = transform * point.to_homogeneous();
    //
    //     Self::new(point.x as T, point.y as T, point.z as T)
    // }

    pub fn cross(&self, other: &Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub(crate) fn transform(&self, transform: &Matrix4<T>) -> Self {
        let mut point = Point3::<T>::new(self.x, self.y, self.z);
        point = transform.transform_point(&point);

        Self::new(point.x as T, point.y as T, point.z as T)
    }

    fn move_coord(&mut self, x: T, y: T, z: T) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    fn scale_coord(&mut self, x: T, y: T, z: T) {
        self.x *= x;
        self.y *= y;
        self.z *= z;
    }

    fn rotate_coord(&mut self, _x: T, _y: T, _z: T) {
        // let x_rad = x.to_radians();
        // let y_rad = y.to_radians();
        // let z_rad = z.to_radians();
        //
        // let x_sin = x_rad.sin();
        // let x_cos = x_rad.cos();
        // let y_sin = y_rad.sin();
        // let y_cos = y_rad.cos();
        // let z_sin = z_rad.sin();
        // let z_cos = z_rad.cos();

        // let x_mat = Matrix4::new(
        //     1.0, 0.0, 0.0, 0.0, 0.0, x_cos, -x_sin, 0.0, 0.0, x_sin, x_cos, 0.0, 0.0, 0.0, 0.0, 1.0,
        // );
        // let y_mat = Matrix4::new(
        //     y_cos, 0.0, y_sin, 0.0, 0.0, 1.0, 0.0, 0.0, -y_sin, 0.0, y_cos, 0.0, 0.0, 0.0, 0.0, 1.0,
        // );
        // let z_mat = Matrix4::new(
        //     z_cos, -z_sin, 0.0, 0.0, z_sin, z_cos, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        // );

        // let mat = x_mat * y_mat * z_mat;

        let (x, y, z) = self.position();
        // let (x, y, z) = mat.transform_point(x, y, z);
        self.set_position(x, y, z);
    }
}

// impl<T> From<(T, T, T)> for Point<T>
// {
//     fn from(value: (T, T, T)) -> Self
//     {
//         Point {
//             x: value.0,
//             y: value.1,
//             z: value.2,
//         }
//     }
// }

impl<K: Into<T>, T> From<(K, K, K)> for Point<T> {
    fn from(value: (K, K, K)) -> Self {
        Point {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
        }
    }
}
impl<T: ops::Add<Output = T>> ops::Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: ops::Neg<Output = T>> Neg for Point<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// impl<T> ops::Mul for Matrix4<T>
// {
//     type Output = Vector4<T>;
//
//     fn mul(self, other: Point<T>)
//     {
//         let vec = Vector4::new(other.x(), other.get_y(), other.get_z(), 1);
//         self * other
//     }
// }
//
impl<T: ops::Mul<Output = T>> ops::Mul for Point<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// impl<T> ops::Mul for Point<T>
// {
//     fn mul(self, other: Matrix4<T>)
//     {
//         let vec = Vector4::new(other.x(), other.get_y(), other.get_z(), 1);
//         let p = vec * other;
//
//         Self {
//             x: p.x,
//             y: p.y,
//             z: p.z,
//         }
//     }
// }

impl<T: ops::Div<Output = T>> ops::Div for Point<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: std::ops::AddAssign> ops::AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: std::ops::SubAssign> ops::SubAssign for Point<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: std::ops::MulAssign> ops::MulAssign for Point<T> {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: std::ops::DivAssign> ops::DivAssign for Point<T> {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

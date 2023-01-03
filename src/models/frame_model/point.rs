use cgmath::{num_traits::Float, BaseFloat, Matrix4, Point3, Transform};
use std::ops;

#[derive(Copy, Clone)]
pub struct Point<T: Float>
{
    x: T,
    y: T,
    z: T,
}

impl<T: BaseFloat + Float + std::ops::MulAssign + std::ops::AddAssign> Point<T>
{
    #[must_use]
    pub const fn new(x: T, y: T, z: T) -> Self
    {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn get_x(&self) -> T
    {
        self.x
    }

    #[must_use]
    pub const fn get_y(&self) -> T
    {
        self.y
    }

    #[must_use]
    pub const fn get_z(&self) -> T
    {
        self.z
    }

    pub fn set_x(&mut self, x: T)
    {
        self.x = x;
    }

    pub fn set_y(&mut self, y: T)
    {
        self.y = y;
    }

    pub fn set_z(&mut self, z: T)
    {
        self.z = z;
    }

    #[must_use]
    pub const fn get_position(&self) -> (T, T, T)
    {
        (self.x, self.y, self.z)
    }

    pub fn set_position(&mut self, x: T, y: T, z: T)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub(crate) fn transform(&self, transform: &Matrix4<T>) -> Self
    {
        let mut point = Point3::<T>::new(self.x, self.y, self.z);
        point = transform.transform_point(point);

        Self::new(point.x as T, point.y as T, point.z as T)
    }

    fn move_coord(&mut self, x: T, y: T, z: T)
    {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    fn scale_coord(&mut self, x: T, y: T, z: T)
    {
        self.x *= x;
        self.y *= y;
        self.z *= z;
    }

    fn rotate_coord(&mut self, _x: T, _y: T, _z: T)
    {
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

        let (x, y, z) = self.get_position();
        // let (x, y, z) = mat.transform_point(x, y, z);
        self.set_position(x, y, z);
    }
}

impl<T: Float> ops::Add for Point<T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Float> ops::Sub for Point<T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
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
//         let vec = Vector4::new(other.get_x(), other.get_y(), other.get_z(), 1);
//         self * other
//     }
// }
//
impl<T: Float> ops::Mul for Point<T>
{
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
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
//         let vec = Vector4::new(other.get_x(), other.get_y(), other.get_z(), 1);
//         let p = vec * other;
//
//         Self {
//             x: p.x,
//             y: p.y,
//             z: p.z,
//         }
//     }
// }

impl<T: Float> ops::Div for Point<T>
{
    type Output = Self;

    fn div(self, other: Self) -> Self
    {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Float + std::ops::AddAssign> ops::AddAssign for Point<T>
{
    fn add_assign(&mut self, other: Self)
    {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Float + std::ops::SubAssign> ops::SubAssign for Point<T>
{
    fn sub_assign(&mut self, other: Self)
    {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Float + std::ops::MulAssign> ops::MulAssign for Point<T>
{
    fn mul_assign(&mut self, other: Self)
    {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: Float + std::ops::DivAssign> ops::DivAssign for Point<T>
{
    fn div_assign(&mut self, other: Self)
    {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

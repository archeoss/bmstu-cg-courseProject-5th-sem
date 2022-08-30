use std::ops;
use cgmath::{Matrix4, Transform};

#[derive(Copy, Clone)]
pub struct Point
{
    x: f32,
    y: f32,
    z: f32,
}

impl Point
{
    pub fn new(x: f32, y: f32, z: f32) -> Point
    {
        Point {x, y, z}
    }

    pub fn get_x(&self) -> f32
    {
        self.x
    }

    pub fn get_y(&self) -> f32
    {
        self.y
    }

    pub fn get_z(&self) -> f32
    {
        self.z
    }

    pub fn set_x(&mut self, x: f32)
    {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32)
    {
        self.y = y;
    }

    pub fn set_z(&mut self, z: f32)
    {
        self.z = z;
    }

    pub fn get_position(&self) -> (f32, f32, f32)
    {
        (self.x, self.y, self.z)
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn move_coord(&mut self, x: f32, y: f32, z: f32)
    {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    fn scale_coord(&mut self, x: f32, y: f32, z: f32)
    {
        self.x *= x;
        self.y *= y;
        self.z *= z;
    }

    fn rotate_coord(&mut self, x: f32, y: f32, z: f32)
    {
        let x_rad = x.to_radians();
        let y_rad = y.to_radians();
        let z_rad = z.to_radians();

        let x_sin = x_rad.sin();
        let x_cos = x_rad.cos();
        let y_sin = y_rad.sin();
        let y_cos = y_rad.cos();
        let z_sin = z_rad.sin();
        let z_cos = z_rad.cos();

        let x_mat = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, x_cos, -x_sin, 0.0,
            0.0, x_sin, x_cos, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let y_mat = Matrix4::new(
            y_cos, 0.0, y_sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            -y_sin, 0.0, y_cos, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let z_mat = Matrix4::new(
            z_cos, -z_sin, 0.0, 0.0,
            z_sin, z_cos, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        let mat = x_mat * y_mat * z_mat;

        let (x, y, z) = self.get_position();
        // let (x, y, z) = mat.transform_point(x, y, z);
        self.set_position(x, y, z);
    }
}

impl ops::Add for Point
{
    type Output = Point;

    fn add(self, other: Point) -> Point
    {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Point
{
    type Output = Point;

    fn sub(self, other: Point) -> Point
    {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul for Point
{
    type Output = Point;

    fn mul(self, other: Point) -> Point
    {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Div for Point
{
    type Output = Point;

    fn div(self, other: Point) -> Point
    {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ops::AddAssign for Point
{
    fn add_assign(&mut self, other: Point)
    {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::SubAssign for Point
{
    fn sub_assign(&mut self, other: Point)
    {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::MulAssign for Point
{
    fn mul_assign(&mut self, other: Point)
    {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl ops::DivAssign for Point
{
    fn div_assign(&mut self, other: Point)
    {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}
use crate::managers::transform_manager::Visitor;
use crate::models::frame_model::Point;
use crate::objects::object::Object;
use crate::objects::visibility::Visibility;
use cgmath::{Matrix4, Vector4};

pub struct Camera
{
    pos: Point<f64>,
    target: Point<f64>,
    up: Point<f64>,
    view_port: (f64, f64),
    fov: f64,
    near: f64,
    far: f64,
}

impl Camera
{
    #[must_use]
    pub fn new(pos: Point<f64>) -> Self
    {
        Self {
            pos,
            target: Point::new(0.0, 0.0, 0.0),
            up: Point::new(0.0, 0.0, 0.0),
            view_port: (800.0, 600.0),
            fov: 90.0,
            near: 1.0,
            far: 100.0,
        }
    }

    pub fn move_camera(&mut self, mov: (f64, f64, f64))
    {
        self.pos += Point::new(mov.0, mov.1, mov.2);
    }

    pub fn pitch(&mut self, grad: f64)
    {
        // let matr = Matrix4::from_angle_x(grad);
    }
}

impl Object for Camera
{
    fn add(&mut self, _obj: Box<dyn Object>) -> bool
    {
        false
    }

    fn remove(&mut self, _obj: Box<dyn Object>) -> bool
    {
        false
    }

    fn accept(&mut self, visitor: &mut dyn Visitor)
    {
        visitor.visit_camera(self)
    }

    fn transform(&mut self, transform: Matrix4<f64>)
    {
        // let vec = Vector4::new(
        //     self.pos.get_x(),
        //     self.pos.get_y(),
        //     self.pos.get_z(),
        //     1.0,
        // );
        // let point: Vector4<f32> = transform * vec;
        // let point: Vector4<f64> = transform * self.pos;
        // self.pos = Point::new(point.x, point.y, point.z);

        // let vec = Vector4::new(self.pos.get_x(), self.pos.get_y(), self.pos.get_z(), 1.0);
    }

    fn transform_first(&mut self, _transform: Matrix4<f64>)
    {
        // self.transform = transform * self.transform;
    }
}

impl Visibility for Camera
{
    fn is_visible(&self) -> bool
    {
        false
    }
}

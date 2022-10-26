use crate::managers::transform_manager::Visitor;
use crate::models::frame_model::Point;
use crate::objects::object::Object;
use crate::objects::visibility::Visibility;
use cgmath::{Matrix4, Vector4};

pub struct Camera
{
    projection_point: Point,
}

impl Camera
{
    pub fn new(projection_point: Point) -> Camera
    {
        Camera { projection_point }
    }
}

impl Object for Camera
{
    fn add(&mut self, obj: Box<dyn Object>) -> bool
    {
        false
    }

    fn remove(&mut self, obj: Box<dyn Object>) -> bool
    {
        false
    }

    fn accept(&mut self, visitor: &mut dyn Visitor)
    {
        visitor.visit_camera(&self)
    }

    fn transform(&mut self, transform: Matrix4<f32>)
    {
        let vec = Vector4::new(
            self.projection_point.get_x(),
            self.projection_point.get_y(),
            self.projection_point.get_z(),
            1.0,
        );
        let point: Vector4<f32> = transform * vec;
        self.projection_point = Point::new(point.x, point.y, point.z);
    }
}

impl Visibility for Camera
{
    fn is_visible(&self) -> bool
    {
        false
    }
}

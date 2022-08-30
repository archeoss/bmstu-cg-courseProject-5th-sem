pub use crate::managers::visitor::Visitor;
use cgmath::{Matrix4, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use crate::models::frame_model::{FrameFigure, FrameModel};
use crate::models::model::Model;

struct TransformManager;

impl TransformManager
{
    fn move_model(&mut self, obj: Rc<RefCell<dyn Model<Output = FrameFigure>>>, dx: f32, dy: f32, dz: f32)
    {
        let transform = Matrix4::from_translation(Vector3::new(dx, dy, dz));

        obj.borrow_mut().transform(transform);
    }

    fn rotate_model(&mut self, obj: Rc<RefCell<dyn Model<Output = FrameFigure>>>, ox: f32, oy: f32, oz: f32)
    {
        let transform = Matrix4::from_angle_x(cgmath::Rad(ox));
        let transform = transform * Matrix4::from_angle_y(cgmath::Rad(oy));
        let transform = transform * Matrix4::from_angle_z(cgmath::Rad(oz));

        obj.borrow_mut().transform(transform);
    }

    fn scale_model(&mut self, obj: Rc<RefCell<dyn Model<Output = FrameFigure>>>, kx: f32, ky: f32, kz: f32)
    {
        let transform = Matrix4::from_nonuniform_scale(kx, ky, kz);

        obj.borrow_mut().transform(transform);
    }
}
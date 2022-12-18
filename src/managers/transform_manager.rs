pub use crate::managers::visitor::Visitor;
use crate::models::frame_model::FrameFigure;
use crate::models::model::Model;
use cgmath::{Matrix4, Vector3};
use std::cell::RefCell;
use std::rc::Rc;

pub struct TransformManager;

impl TransformManager
{
    #[must_use] pub fn new() -> Self
    {
        Self
    }

    pub fn move_model(
        &mut self,
        obj: Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>,
        dx: f32,
        dy: f32,
        dz: f32,
    )
    {
        let transform = Matrix4::from_translation(Vector3::new(dx, dy, dz));

        obj.borrow_mut().transform(transform);
    }

    pub fn rotate_model(
        &mut self,
        obj: Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>,
        ox: f32,
        oy: f32,
        oz: f32,
    )
    {
        let center = obj.borrow().get_center();

        let transform = Matrix4::from_translation(Vector3::new(
            -center.get_x(),
            -center.get_y(),
            -center.get_z(),
        )) * Matrix4::from_angle_x(cgmath::Rad(ox))
            * Matrix4::from_angle_y(cgmath::Rad(oy))
            * Matrix4::from_angle_z(cgmath::Rad(oz))
            * Matrix4::from_translation(Vector3::new(
                center.get_x(),
                center.get_y(),
                center.get_z(),
            ));
        // let transform = Matrix4::from_angle_x(cgmath::Rad(ox));
        // let transform = transform * Matrix4::from_angle_y(cgmath::Rad(oy));
        // let transform = transform * Matrix4::from_angle_z(cgmath::Rad(oz));

        obj.borrow_mut().transform(transform);
    }

    pub fn scale_model(
        &mut self,
        obj: Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>,
        kx: f32,
        ky: f32,
        kz: f32,
    )
    {
        let transform = Matrix4::from_nonuniform_scale(kx, ky, kz);

        obj.borrow_mut().transform(transform);
    }
}

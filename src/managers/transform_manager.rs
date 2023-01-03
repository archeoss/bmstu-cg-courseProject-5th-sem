pub use crate::managers::visitor::Visitor;
use crate::models::frame_model::FrameFigure;
use crate::models::model::Model;
use cgmath::{Matrix4, Vector3};
use std::cell::RefCell;
use std::rc::Rc;

pub struct TransformManager
{
    to_transform: Vec<bool>,
}

impl TransformManager
{
    #[must_use]
    pub fn new() -> Self
    {
        Self {
            to_transform: Vec::new(),
        }
    }

    pub fn expand(&mut self)
    {
        self.to_transform.push(false);
    }

    pub fn remove(&mut self, index: usize)
    {
        self.to_transform.remove(index);
    }

    pub fn get_to_transform(&mut self) -> &mut [bool]
    {
        self.to_transform.as_mut_slice()
    }

    pub fn move_model(
        &mut self,
        obj: Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>,
        dx: f64,
        dy: f64,
        dz: f64,
    )
    {
        let transform = Matrix4::from_translation(Vector3::new(dx, dy, dz));

        println!("Transform: {:?}", transform);
        obj.borrow_mut().transform_first(transform);
    }

    pub fn rotate_model(
        &mut self,
        obj: Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>,
        ox: f64,
        oy: f64,
        oz: f64,
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
        kx: f64,
        ky: f64,
        kz: f64,
    )
    {
        let transform = Matrix4::from_nonuniform_scale(kx, ky, kz);

        obj.borrow_mut().transform(transform);
    }

    pub fn move_models(
        &mut self,
        models: &mut [Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>],
        mv: (f64, f64, f64),
    )
    {
        if models.len() == 0 {
            return;
        }
        let transform = Matrix4::from_translation(Vector3::new(mv.0, mv.1, mv.2));

        println!("Transform: {:?}", transform);
        for model in models.iter_mut() {
            model.borrow_mut().transform_first(transform);
        }
    }

    pub fn rotate_models(
        &mut self,
        models: &mut [Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>],
        rot: (f64, f64, f64),
    )
    {
        if models.len() == 0 {
            return;
        }

        let center = models[0].borrow().get_center();

        let transform = Matrix4::from_translation(Vector3::new(
            -center.get_x(),
            -center.get_y(),
            -center.get_z(),
        )) * Matrix4::from_angle_x(cgmath::Rad(rot.0))
            * Matrix4::from_angle_y(cgmath::Rad(rot.1))
            * Matrix4::from_angle_z(cgmath::Rad(rot.2))
            * Matrix4::from_translation(Vector3::new(
                center.get_x(),
                center.get_y(),
                center.get_z(),
            ));

        for model in models.iter_mut() {
            model.borrow_mut().transform(transform);
        }
    }

    pub fn scale_models(
        &mut self,
        models: &mut [Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>],
        scale: (f64, f64, f64),
    )
    {
        if models.len() == 0 {
            return;
        }
        let transform = Matrix4::from_nonuniform_scale(scale.0, scale.1, scale.2);

        for model in models.iter_mut() {
            model.borrow_mut().transform(transform);
        }
    }
}

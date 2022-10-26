use crate::models::frame_model::Point;
use cgmath::Matrix4;
use std::cell::RefCell;
use std::rc::Rc;
// use crate::models::frame_model::FrameModel;

pub trait Model
{
    type Output;

    fn get_model(&self) -> Rc<RefCell<Self::Output>>;
    fn get_center(&self) -> Point;
    fn get_transform(&self) -> Matrix4<f32>;
    fn transform(&mut self, transform: Matrix4<f32>);
}

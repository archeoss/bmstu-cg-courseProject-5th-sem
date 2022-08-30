use std::cell::RefCell;
use std::rc::Rc;
use cgmath::Matrix4;
// use crate::models::frame_model::FrameModel;

pub trait Model
{
    type Output;

    fn get_model(&self) -> Rc<RefCell<Self::Output>>;
    fn transform(&mut self, transform: Matrix4<f32>);
}

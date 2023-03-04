use crate::models::frame_model::Point;
use nalgebra::Matrix4;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
// use crate::models::frame_model::FrameModel;

pub trait ModelToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> ModelToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait Model: ModelToAny {
    type Output;

    fn figures(&self) -> Vec<Rc<RefCell<Self::Output>>>;
    fn add_figure(&mut self, model: Rc<RefCell<Self::Output>>);
    fn center(&self) -> Point<f64>;
    fn true_center(&self) -> Point<f64>;
    fn name(&self) -> String;
    fn set_name(&mut self, name: &str);
    fn transform(&self) -> Matrix4<f64>;
    fn transform_self(&mut self, transform: Matrix4<f64>);
    fn transform_first(&mut self, transform: Matrix4<f64>);
    fn update_model(&mut self);
}

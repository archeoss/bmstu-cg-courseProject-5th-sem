use crate::{
    custom_loader::builder_factory::model_builder::FrameModelBuilder,
    models::{
        frame_model::{Edge, FrameFigure, FrameModel, Point},
        model::Model,
    },
};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};
const H_ANGLE: f64 = PI / 180.0 * 72.0;
// const V_ANGLE: f64 = (0.5f64).atan();

#[derive(Debug)]
pub struct AbstractModel {
    frame_model: Rc<RefCell<FrameModel>>,
}

impl AbstractModel {
    pub fn new(frame_model: FrameModel) -> Self {
        Self {
            frame_model: Rc::new(RefCell::new(frame_model)),
        }
    }
}

impl Model for AbstractModel {
    type Output = FrameModel;

    fn figures(&self) -> Vec<Rc<RefCell<Self::Output>>> {
        vec![self.frame_model.clone()]
    }

    fn true_center(&self) -> Point<f64> {
        self.frame_model.borrow().true_center()
    }

    fn center(&self) -> Point<f64> {
        self.frame_model.borrow().center()
    }

    fn name(&self) -> String {
        self.frame_model.borrow().name()
    }

    fn transform(&self) -> nalgebra::Matrix4<f64> {
        self.frame_model.borrow().transform()
    }

    fn transform_self(&mut self, transform: nalgebra::Matrix4<f64>) {
        self.frame_model.borrow_mut().transform_self(transform);
    }

    fn transform_first(&mut self, transform: nalgebra::Matrix4<f64>) {
        self.frame_model.borrow_mut().transform_first(transform);
    }

    fn update_model(&mut self) {
        self.frame_model.borrow_mut().update_model();
    }

    fn add_figure(&mut self, model: Rc<RefCell<Self::Output>>) {
        for figure in model.borrow().figures() {
            self.frame_model.borrow_mut().add_figure(figure);
        }
    }

    fn set_name(&mut self, name: &str) {
        self.frame_model.borrow_mut().set_name(name);
    }
}

impl From<Rc<RefCell<FrameModel>>> for AbstractModel {
    fn from(frame_model: Rc<RefCell<FrameModel>>) -> Self {
        Self { frame_model }
    }
}

impl From<AbstractModel> for Rc<RefCell<FrameModel>> {
    fn from(sphere: AbstractModel) -> Self {
        sphere.figures()[0].clone()
    }
}

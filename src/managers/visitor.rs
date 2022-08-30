use crate::models::frame_model::{FrameFigure, FrameModel};
use crate::models::model::Model;
pub trait Visitor
{
    fn visit_model(&mut self, obj: &dyn Model<Output = FrameFigure>);
    // fn visit_
}
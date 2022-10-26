use crate::models::frame_model::FrameFigure;
use crate::models::model::Model;
use crate::objects::camera::Camera;

pub trait Visitor
{
    fn visit_model(&mut self, obj: &dyn Model<Output = FrameFigure>);
    fn visit_camera(&mut self, obj: &Camera);
}

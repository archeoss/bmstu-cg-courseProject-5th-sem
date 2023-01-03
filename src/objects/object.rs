use crate::managers::visitor::Visitor;
use cgmath::Matrix4;

pub trait Object
{
    fn add(&mut self, obj: Box<dyn Object>) -> bool;
    fn remove(&mut self, obj: Box<dyn Object>) -> bool;
    fn accept(&mut self, visitor: &mut dyn Visitor);
    fn transform(&mut self, transform: Matrix4<f64>);
    fn transform_first(&mut self, transform: Matrix4<f64>);
    // fn get_type(&self) -> ObjectType;
    // fn inspect(&self) -> String;
}

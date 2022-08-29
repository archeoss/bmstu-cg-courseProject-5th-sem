use crate::models::model::Model;
pub trait Visitor<T>
{
    fn visit_model(&mut self, obj: &dyn Model);
    // fn visit_
}
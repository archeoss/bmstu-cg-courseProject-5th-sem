pub trait Object {
    fn add(&mut self, obj: Box<dyn Object>) -> bool;
    fn remove(&mut self, obj: Box<dyn Object>) -> bool;
    fn accept(&mut self, visitor: &mut dyn Visitor);
    fn transform(&mut self, transform: &Transform);
    // fn get_type(&self) -> ObjectType;
    // fn inspect(&self) -> String;
}

pub trait MainApp
{
    fn new(width: u32, height: u32) -> Box<dyn MainApp> where Self: Sized;
    fn launch(&self);
}
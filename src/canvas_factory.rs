pub mod canvas;
use canvas::Canvas;
use canvas::CanvasSDL;
trait Factory
{
    fn make(&self, x: u32, y: u32) -> Box<dyn Canvas>;
}

struct SDLFactory;
impl Factory for SDLFactory
{
    fn make(&self, x: u32, y: u32) -> Box<dyn Canvas>
    {
        Box::from(CanvasSDL::new(x, y))
    }
}

pub fn create_window(interface: &'static str) -> Result<Box<dyn Canvas>, &'static str>
{
    match interface
    {
        "sdl" =>
        {
            let factory: Box<dyn Factory> = Box::new(SDLFactory {});
            let canvas = factory.make(600, 600);
            Ok(canvas)
        }
        _ =>
        {
            // panic!("Unknown interface");
            Err("Unknown interface")
        }
    }
}
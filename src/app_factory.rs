pub mod app_winit;
// pub mod app_sdl; //TODO
pub mod app;
pub mod canvas;

use app::App;
use app_winit::AppPixel;

trait AppFactory
{
    fn make(&self, width: u32, height: u32) -> Box<dyn App>;
}

pub struct WinitFactory;
impl AppFactory for WinitFactory
{
    fn make(&self, width: u32, height: u32) -> Box<dyn App>
    {
        Box::from(AppPixel::new(width, height))
    }
}
// TODO
// pub struct SDLFactory;
// impl AppFactory for SDLFactory
// {
//     fn make(&self, width: u32, height: u32) -> Box<dyn App>
//     {
//         Box::from(CanvasSDL::new(width, height))
//     }
// }

pub fn create_app(interface: &'static str) -> Result<Box<dyn App>, &'static str>
{
    match interface
    {
        // TODO
        // "sdl" =>
        //     {
        //         let factory: Box<dyn Factory> = Box::new(SDLFactory {});
        //         let canvas = factory.make(600, 600);
        //         Ok(canvas)
        //     }
        "winit-pixel" =>
            {
                let factory: Box<dyn AppFactory> = Box::new(WinitFactory {});
                let app = factory.make(640, 480);
                Ok(app)
            }
        _ =>
            {
                // panic!("Unknown interface");
                Err("Unknown interface")
            }
    }
}

pub mod canvas_pixel;

use std::error::Error;
use async_trait::async_trait;
// pub mod canvas_sdl;  //TODO
use canvas_pixel::CanvasPixel;
use errors::notImplError::NotImplError;
use crate::app_factory::errors;

#[async_trait]
pub trait Canvas
{
    async fn new(width: u32, height: u32, init_frame: &[u8]) -> Self where Self: Sized;
    fn point(&mut self, x: i32, y: i32, color: [u8; 4]);
    fn copy_to_buffer(&mut self, surface: &mut [u8]);
    fn get_frame(&mut self) -> &mut [u8];
    fn resize_surface(&mut self, width: u32, height: u32, new_frame: &[u8]);
}

#[async_trait]
trait CanvasFactory
{
    type Output;
    async fn make(&self, width: u32, height: u32, init_frame: &[u8]) -> Self::Output;
}

pub struct CanvasPixelFactory;
#[async_trait]
impl CanvasFactory for CanvasPixelFactory
{
    type Output = Box<CanvasPixel>;

    async fn make(&self, width: u32, height: u32, init_frame: &[u8])
                  -> Self::Output { Box::new((CanvasPixel::new(width, height, init_frame)).await) }
}

pub async fn create_canvas
    (interface: &'static str, width: u32, height: u32, init_frame: &[u8])
                           -> Result<Box<dyn Canvas>, Box<dyn Error>>
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
        "pixel" =>
            {
                let factory: Box<dyn CanvasFactory<Output = Box<CanvasPixel>>> = Box::new(CanvasPixelFactory {});
                let canvas = factory.make(width, height, init_frame);
                Ok(canvas.await)
            }
        _ =>
            {
                // panic!("Unknown interface");
                Err(Box::new(NotImplError::new(interface)))
            }
    }
}

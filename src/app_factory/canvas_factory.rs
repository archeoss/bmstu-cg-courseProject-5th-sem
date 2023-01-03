use std::error::Error;
pub mod canvas;
pub mod canvas_skia;
use crate::errors;
use canvas::Canvas;
use canvas_skia::CanvasSkia;
use errors::not_impl_error::NotImplError;

// #[async_trait]
trait CanvasFactory
{
    type Output;
    /*async*/
    fn make(&self, width: u32, height: u32) -> Self::Output;
}

pub struct CanvasSkiaFactory;
// #[async_trait]
impl CanvasFactory for CanvasSkiaFactory
{
    type Output = Box<CanvasSkia>;

    /*async*/
    fn make(&self, width: u32, height: u32) -> Self::Output
    {
        Box::new(CanvasSkia::new(width, height) /*.await*/)
    }
}

pub fn create_canvas(
    interface: &'static str,
    width: u32,
    height: u32,
) -> Result<Box<dyn Canvas>, Box<dyn Error>>
{
    match interface {
        "skia" => {
            let factory: Box<dyn CanvasFactory<Output = Box<CanvasSkia>>> =
                Box::new(CanvasSkiaFactory {});
            let canvas = factory.make(width, height);
            Ok(canvas)
        }
        _ => Err(Box::new(NotImplError::new(interface))),
    }
}

use std::error::Error;
pub mod canvas;
pub mod canvas_skia;
use crate::errors;
use canvas::Canvas;
use canvas_skia::CanvasSkia;
use errors::not_impl_error::NotImplError;
use nalgebra::Point3;

pub struct Section
{
    pub y_start: i64,
    pub y_end: i64,
    pub x_start: f64,
    pub x_step: f64,
    pub z_start: f64,
    pub z_step: f64,
    pub br_start: f64,
    pub br_step: f64,
}

impl Section
{
    pub fn new(from: &Point3<i64>, to: &Point3<i64>, from_br: f64, to_br: f64) -> Self
    {
        let diff_y = to.y - from.y;
        Self {
            y_start: from.y,
            y_end: to.y,
            x_start: from.x as f64,
            z_start: from.z as f64,
            x_step: (to.x - from.x) as f64 / diff_y as f64,
            z_step: (to.z - from.z) as f64 / diff_y as f64,
            br_start: from_br,
            br_step: (to_br - from_br) / diff_y as f64,
        }
    }
}
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

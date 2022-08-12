use std::error::Error;
use std::sync::{Arc, Mutex};
use crate::app_factory::canvas::Canvas;
// use errors::notImplError::NotImplError;
use crate::app_factory::errors::notImplError::NotImplError;

pub mod drawerSTD;
use drawerSTD::DrawerSTD;
pub trait Drawer
{
    fn set_canvas(&mut self, canvas: Arc<Mutex<Box<dyn Canvas>>>);
    fn draw_point(&mut self, x: i32, y: i32, color: [u8; 4]);
    fn draw_line(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32, color: [u8; 4]);
    fn draw_line_AA(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32, color: [u8; 4]);
    fn draw_ellipse(&mut self, x: i32, y: i32, width: i32, height: i32, color: [u8; 4]);
}

trait DrawerFactory
{
    fn make(&self, canvas: Arc<Mutex<Box<dyn Canvas>>>) -> Box<dyn Drawer>;
}

pub struct FactoryDrawerSTD;

impl DrawerFactory for FactoryDrawerSTD
{
    fn make(&self, canvas: Arc<Mutex<Box<dyn Canvas>>>) -> Box<dyn Drawer>
    {
        Box::new(DrawerSTD::new(canvas))
    }
}

pub fn create_drawer(interface: &'static str, canvas: Arc<Mutex<Box<dyn Canvas>>>) -> Result<Box<dyn Drawer>, Box<dyn Error>>
{
    match interface
    {
        // "sdl" =>     // TODO
        // {
        //     let factory: Box<dyn Factory> = Box::new(SDLFactory {});
        //     let canvas = factory.make(600, 600);
        //     Ok(canvas)
        // }
        "std" =>
        {
            let factory: Box<dyn DrawerFactory> = Box::new(FactoryDrawerSTD {});
            let drawer = factory.make(canvas);
            Ok(drawer)
        }
        _ =>
        {
            // panic!("Unknown interface");
            Err(Box::new(NotImplError::new(interface)))
        }
    }
}
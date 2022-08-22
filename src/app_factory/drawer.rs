
use std::error::Error;
use std::sync::{Arc, Mutex};
// use errors::notImplError::NotImplError;
use crate::errors::not_impl_error::NotImplError;

pub mod drawer_std;
use drawerSTD::DrawerSTD;
use crate::app_factory::canvas_factory::canvas::Canvas;

pub trait Drawer {
    fn set_canvas(&mut self, canvas: Arc<Mutex<Box<dyn Canvas>>>);
    fn draw_point(&mut self, x: i32, y: i32, color: [u8; 4]);
    fn draw_line(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32, color: [u8; 4]);
    fn draw_line_aa(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32, color: [u8; 4]);
    fn draw_ellipse(&mut self, x: i32, y: i32, width: i32, height: i32, color: [u8; 4]);
}

trait DrawerFactory {
    fn make(&self, canvas: Arc<Mutex<Box<dyn Canvas>>>) -> Box<dyn Drawer>;
}

pub struct FactoryDrawerSTD;

impl DrawerFactory for FactoryDrawerSTD {
    fn make(&self, canvas: Arc<Mutex<Box<dyn Canvas>>>) -> Box<dyn Drawer> {
        Box::new(DrawerSTD::new(canvas))
    }
}

pub fn create_drawer(
    interface: &'static str,
    canvas: Arc<Mutex<Box<dyn Canvas>>>,
) -> Result<Box<dyn Drawer>, Box<dyn Error>> {
    match interface {
        // "sdl" =>     // TODO
        // {
        //     let factory: Box<dyn Factory> = Box::new(SDLFactory {});
        //     let canvas_factory = factory.make(600, 600);
        //     Ok(canvas_factory)
        // }
        "std" => {
            let factory: Box<dyn DrawerFactory> = Box::new(FactoryDrawerSTD {});
            let drawer = factory.make(canvas);
            Ok(drawer)
        }
        _ => {
            // panic!("Unknown interface");
            Err(Box::new(NotImplError::new(interface)))
        }
    }
}

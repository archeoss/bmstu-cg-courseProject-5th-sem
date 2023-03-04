use crate::errors::not_impl_error::NotImplError;
use crate::models::frame_model::{FrameModel, Point};
use crate::objects::camera::Camera;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[allow(clippy::module_name_repetitions)]
pub mod drawer_std;
use crate::app_factory::canvas_factory::canvas::Canvas;
use crate::models::model::Model;
use drawer_std::DrawerSTD;

pub trait Drawer
{
    fn set_canvas(&mut self, canvas: Rc<RefCell<Box<dyn Canvas>>>);
    fn draw_point(&mut self, x: i32, y: i32, color: [u8; 4]);
    fn draw_line(&mut self, start: (i32, i32, i32), end: (i32, i32, i32), color: [u8; 4]);
    fn draw_line_aa(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32, color: [u8; 4]);
    fn draw_ellipse(&mut self, x: i32, y: i32, width: i32, height: i32, color: [u8; 4]);
    fn copy_to(&self, buffer: &mut [u8]);
    fn fill(&mut self, color: [u8; 4]);
    fn frame(&self) -> Vec<u8>;
    fn set_camera(&mut self, cam: Rc<RefCell<Camera>>);
}

#[allow(clippy::module_name_repetitions)]
pub trait FrameDrawer: Drawer
{
    fn draw_frame_model(
        &mut self,
        frame_models: &[Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>],
    );
    fn draw_in_3d(
        &mut self,
        frame_models: &[Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>],
        light: Point<f64>,
    );
}

trait DrawerFactory<Trait>
{
    fn make(&self, canvas: Rc<RefCell<Box<dyn Canvas>>>) -> Box<Trait>;
}

pub struct FactoryDrawerSTD;

impl DrawerFactory<DrawerSTD> for FactoryDrawerSTD
{
    fn make(&self, canvas: Rc<RefCell<Box<dyn Canvas>>>) -> Box<DrawerSTD>
    {
        Box::new(DrawerSTD::new(canvas))
    }
}

// I didn't find a way to implement this via single function
// I'm dumb

#[allow(clippy::module_name_repetitions)]
pub fn create_drawer(
    interface: &'static str,
    canvas: Rc<RefCell<Box<dyn Canvas>>>,
) -> Result<Box<dyn Drawer>, Box<dyn Error>>
{
    match interface {
        "std" => {
            let factory: Box<dyn DrawerFactory<DrawerSTD>> = Box::new(FactoryDrawerSTD {});
            let drawer = factory.make(canvas);
            Ok(drawer)
        }
        _ => {
            // panic!("Unknown interface");
            Err(Box::new(NotImplError::new(interface)))
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn create_frame_drawer(
    interface: &'static str,
    canvas: Rc<RefCell<Box<dyn Canvas>>>,
) -> Result<Box<dyn FrameDrawer>, Box<dyn Error>>
{
    match interface {
        "std" => {
            let factory: Box<dyn DrawerFactory<DrawerSTD>> = Box::new(FactoryDrawerSTD {});
            let drawer = factory.make(canvas);
            Ok(drawer)
        }
        _ => {
            // panic!("Unknown interface");
            Err(Box::new(NotImplError::new(interface)))
        }
    }
}

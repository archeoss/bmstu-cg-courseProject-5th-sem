use crate::errors::not_impl_error::NotImplError;
use std::error::Error;
// pub mod slint_mod;
pub mod app;
pub mod app_slint;
pub mod canvas_factory;
pub mod drawer;
use crate::app_factory::app::MainApp;
use crate::app_factory::app_slint::SlintApp;

// use {MainApp, SlintApp};
slint::include_modules!();

pub trait AppFactory
{
    fn make(&self, width: u32, height: u32) -> &mut Box<dyn MainApp>;
}

pub struct SlintFactory;
impl AppFactory for SlintFactory
{
    fn make(&self, width: u32, height: u32) -> &mut Box<dyn MainApp>
    {
        Box::leak(Box::from(SlintApp::new(width, height)))
    }
}

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn create_app(interface: &'static str) -> Result<&mut Box<dyn AppFactory>, Box<dyn Error>>
{
    match interface {
        // TODO
        // "winit-pixel" => {
        //     let factory: Box<dyn AppFactory> = Box::new(WinitFactory {});
        //     let app = factory.make(640, 480);
        //     Ok(app)
        // }
        "slint-skia" => {
            let factory: &mut Box<dyn AppFactory> = Box::leak(Box::new(Box::new(SlintFactory {})));
            Ok(factory)
        }
        _ => {
            // panic!("Unknown interface");
            Err(Box::new(NotImplError::new(interface)))
        }
    }
}

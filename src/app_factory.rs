use std::error::Error;
use crate::errors::not_impl_error::NotImplError;
// pub mod slint_mod;
pub mod app;
pub mod app_slint;
pub mod canvas_factory;
pub mod drawer;
use crate::app_factory::app::MainApp;
use crate::app_factory::app_slint::SlintApp;

// use {MainApp, SlintApp};
slint::include_modules!();

trait AppFactory {
    fn make(&self, width: u32, height: u32) -> Box<dyn MainApp>;
}

pub struct SlintFactory;
impl AppFactory for SlintFactory {
    fn make(&self, width: u32, height: u32) -> Box<dyn MainApp> {
        Box::from(SlintApp::new(width, height))
    }
}

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn create_app(interface: &'static str) -> Result<Box<dyn MainApp>, Box<dyn Error>> {
    match interface {
        // TODO
        // "winit-pixel" => {
        //     let factory: Box<dyn AppFactory> = Box::new(WinitFactory {});
        //     let app = factory.make(640, 480);
        //     Ok(app)
        // }
        "slint-skia" => {
            let factory: Box<dyn AppFactory> = Box::new(SlintFactory {});
            let app = factory.make(640, 480);
            println!("kek");
            Ok(app)
        }
        _ => {
            // panic!("Unknown interface");
            Err(Box::new(NotImplError::new(interface)))
        }
    }
}

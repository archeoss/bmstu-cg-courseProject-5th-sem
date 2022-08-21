slint::include_modules!();
use std::sync::{Arc, Mutex};
use crate::app_factory::app::{MainApp};
use crate::app_factory::canvas_factory::create_canvas;
use crate::app_factory::drawer::create_drawer;

pub struct SlintApp
{
    width: u32,
    height: u32,
}

impl MainApp for SlintApp
{
    fn new(width: u32, height: u32) -> Box<dyn MainApp> {
        Box::new(SlintApp {
            width,
            height,
        })
    }

    fn launch(&self)
    {
        let ui = AppWindow::new();

        let ui_handle = ui.as_weak();
        ui.on_request_increase_value(move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        });
        let canvas = create_canvas("skia", self.width, self.height ).unwrap();
        let drawer = create_drawer("std", Arc::new(Mutex::new(canvas))).unwrap();
        ui.run();
    }
}
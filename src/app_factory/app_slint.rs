slint::include_modules!();
use std::cell::RefCell;
use std::rc::Rc;


use std::future::Future;
// use std::sync::{Arc, Mutex};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use crate::models::frame_model::{FrameFigure, FrameModel};
use crate::app_factory::canvas_factory::canvas::Canvas;
use crate::app_factory::app::{MainApp};
use crate::app_factory::canvas_factory::create_canvas;
use crate::app_factory::drawer::{create_drawer, create_frame_drawer, Drawer, FrameDrawer};
use crate::managers::load_manager::{LoadManager};
use crate::managers::transform_manager::TransformManager;
use crate::models::model::Model;

pub struct SlintApp
{
    width: u32,
    height: u32,
    canvas: Rc<RefCell<Box<dyn Canvas>>>,
    drawer: Rc<RefCell<Box<dyn FrameDrawer>>>,
    models: Rc<RefCell<Vec<Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>>>>,
}

impl SlintApp
{
    fn set_canvas(&mut self, canvas: Rc<RefCell<Box<dyn Canvas>>>)
    {
        self.canvas = canvas;
    }
    fn set_drawer(&mut self, drawer: Rc<RefCell<Box<dyn FrameDrawer>>>)
    {
        self.drawer = drawer;
    }


}

impl MainApp for SlintApp
{
    fn new(width: u32, height: u32) -> Box<dyn MainApp> {

        let canvas = Rc::new(RefCell::new(create_canvas("skia", width, height/*self.width, self.height*/ ).unwrap()));
        let drawer = Rc::new(RefCell::new(create_frame_drawer("std", canvas.clone()).unwrap()));

        Box::new(
            SlintApp {
            width,
            height,
            // canvas: None,
            // drawer: None
            canvas,//: Some(canvas),
            drawer,//: Some(drawer)
            models: Rc::new(RefCell::new(Vec::new())),
        })
    }

    fn launch(&'static mut self)
    {
        let ui = AppWindow::new();

        let ui_handle = ui.as_weak();
        ui.on_request_increase_valuee(move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        });
        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(750, 800);
        let width = pixel_buffer.width();
        let height = pixel_buffer.height();
        let mut pixmap = tiny_skia::PixmapMut::from_bytes(
            pixel_buffer.make_mut_bytes(), width, height
        ).unwrap();
        pixmap.fill(tiny_skia::Color::BLACK);
        ui.set_canvas(Image::from_rgba8_premultiplied(pixel_buffer));

        let ui_handle = ui.as_weak();
        let mut on_draw_models = self.models.clone();
        let canvas = Rc::new(RefCell::new(create_canvas("skia", width, height/*self.width, self.height*/ ).unwrap()));
        let drawer = Rc::new(RefCell::new(create_frame_drawer("std", canvas.clone()).unwrap()));

        let mut draww = self.drawer.clone();
        // let mut draww = dr.borrow_mut();

        ui.on_draw_frame_model(move || {

            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(750, 800);
            let width = pixel_buffer.width();
            let height = pixel_buffer.height();
            let mut pixmap = tiny_skia::PixmapMut::from_bytes(
                pixel_buffer.make_mut_bytes(), width, height
            ).unwrap();
            draww.borrow_mut().fill([0, 0, 0, 255]);
            pixmap.fill(tiny_skia::Color::BLACK);

            let ui = ui_handle.unwrap();
            draww.borrow_mut().draw_line(100, 100, 400, 400, [0, 255, 0, 255]);
            println!("bababa");
            let mut load_manager = LoadManager::new();
            // let mut transform_manager = TransformManager::new();

            let path = std::env::current_dir().unwrap();

            let task = rfd::FileDialog::new()
                .set_directory(&path)
                .pick_file();
            let mut model = load_manager.load(task.unwrap().to_str().unwrap(), "frame").unwrap();
            let mut model = Rc::new(RefCell::new(model));
            on_draw_models.borrow_mut().push(model);
            // transform_manager.move_model(&model, 100.0,210.0,10.0);
            // transform_manager.rotate_model(&model, 30.0, 0.0, 0.0);
            draww.borrow_mut().draw_frame_model(on_draw_models.borrow_mut()[0].clone());
            pixmap.data_mut().copy_from_slice(draww.borrow_mut().get_frame().as_slice());
            let image =
                Image::from_rgba8_premultiplied(pixel_buffer);
            ui.set_canvas(image);
            });
        let ui_handle2 = ui.as_weak();
        let mut draww2 = self.drawer.clone();
        let mut on_move_models = self.models.clone();
        ui.on_move_model(move || {
            let ui = ui_handle2.unwrap();

            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(750, 800);
            let width = pixel_buffer.width();
            let height = pixel_buffer.height();
            let mut pixmap = tiny_skia::PixmapMut::from_bytes(
                pixel_buffer.make_mut_bytes(), width, height
            ).unwrap();
            draww2.borrow_mut().fill([0, 0, 0, 255]);
            pixmap.fill(tiny_skia::Color::BLACK);

            let mut transform_manager = TransformManager::new();

            transform_manager.move_model(on_move_models.borrow_mut()[0].clone(), 100.0,210.0,10.0);

            draww2.borrow_mut().draw_frame_model(on_move_models.borrow_mut()[0].clone());
            pixmap.data_mut().copy_from_slice(draww2.borrow_mut().get_frame().as_slice());
            let image =
                Image::from_rgba8_premultiplied(pixel_buffer);
            ui.set_canvas(image);
        });
        let ui_handle3 = ui.as_weak();
        let mut draww3 = self.drawer.clone();
        let mut on_rotate_models = self.models.clone();
        ui.on_rotate_model(move || {
            let ui = ui_handle3.unwrap();

            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(750, 800);
            let width = pixel_buffer.width();
            let height = pixel_buffer.height();
            let mut pixmap = tiny_skia::PixmapMut::from_bytes(
                pixel_buffer.make_mut_bytes(), width, height
            ).unwrap();
            draww3.borrow_mut().fill([0, 0, 0, 255]);
            pixmap.fill(tiny_skia::Color::BLACK);

            let mut transform_manager = TransformManager::new();

            transform_manager.rotate_model(on_rotate_models.borrow_mut()[0].clone(), 100.0,210.0,10.0);

            draww3.borrow_mut().draw_frame_model(on_rotate_models.borrow_mut()[0].clone());
            pixmap.data_mut().copy_from_slice(draww3.borrow_mut().get_frame().as_slice());
            let image =
                Image::from_rgba8_premultiplied(pixel_buffer);
            ui.set_canvas(image);
        });
        let ui_handle4 = ui.as_weak();
        let mut draww4 = self.drawer.clone();
        let mut on_scale_models = self.models.clone();
        ui.on_scale_model(move || {
            let ui = ui_handle4.unwrap();

            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(750, 800);
            let width = pixel_buffer.width();
            let height = pixel_buffer.height();
            let mut pixmap = tiny_skia::PixmapMut::from_bytes(
                pixel_buffer.make_mut_bytes(), width, height
            ).unwrap();
            draww4.borrow_mut().fill([0, 0, 0, 255]);
            pixmap.fill(tiny_skia::Color::BLACK);

            let mut transform_manager = TransformManager::new();

            transform_manager.scale_model(on_scale_models.borrow_mut()[0].clone(), 100.0,210.0,10.0);

            draww4.borrow_mut().draw_frame_model(on_scale_models.borrow_mut()[0].clone());
            pixmap.data_mut().copy_from_slice(draww4.borrow_mut().get_frame().as_slice());
            let image =
                Image::from_rgba8_premultiplied(pixel_buffer);
            ui.set_canvas(image);
        });
            ui.run();
    }
}
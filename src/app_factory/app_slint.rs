slint::include_modules!();
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
// use std::sync::{Arc, Mutex};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use crate::models::frame_model::FrameModel;
use crate::app_factory::canvas_factory::canvas::Canvas;
use crate::app_factory::app::{MainApp};
use crate::app_factory::canvas_factory::create_canvas;
use crate::app_factory::drawer::{create_drawer, Drawer};
use crate::managers::load_manager::{LoadManager};

pub struct SlintApp
{
    width: u32,
    height: u32,
    canvas: Option<Rc<RefCell<Box<dyn Canvas>>>>,
    drawer: Option<Rc<RefCell<Box<dyn Drawer>>>>
}

impl SlintApp
{
    fn set_canvas(&mut self, canvas: Rc<RefCell<Box<dyn Canvas>>>)
    {
        self.canvas = Some(canvas);
    }
    fn set_drawer(&mut self, drawer: Rc<RefCell<Box<dyn Drawer>>>)
    {
        self.drawer = Some(drawer);
    }
}

impl MainApp for SlintApp
{
    fn new(width: u32, height: u32) -> Box<dyn MainApp> {
        Box::new(SlintApp {
            width,
            height,
            canvas: None,
            drawer: None
        })
    }

    fn launch(&mut self)
    {
        let ui = AppWindow::new();

        let ui_handle = ui.as_weak();
        ui.on_request_increase_valuee(move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        });
        //
        // let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(750, 800);
        // let width = pixel_buffer.width();
        // let height = pixel_buffer.height();
        // let mut pixmap = tiny_skia::PixmapMut::from_bytes(
        //     pixel_buffer.make_mut_bytes(), width, height
        // ).unwrap();
        // pixmap.fill(tiny_skia::Color::BLACK);

        // let circle = tiny_skia::PathBuilder::from_circle(320., 240., 150.).unwrap();
        //
        // let mut paint = tiny_skia::Paint::default();
        // paint.shader = tiny_skia::LinearGradient::new(
        //     tiny_skia::Point::from_xy(100.0, 100.0),
        //     tiny_skia::Point::from_xy(400.0, 400.0),
        //     vec![
        //         tiny_skia::GradientStop::new(0.0, tiny_skia::Color::from_rgba8(50, 127, 150, 200)),
        //         tiny_skia::GradientStop::new(1.0, tiny_skia::Color::from_rgba8(220, 140, 75, 180)),
        //     ],
        //     tiny_skia::SpreadMode::Pad,
        //     tiny_skia::Transform::identity(),
        // ).unwrap();
        // self.set_canvas(Rc::new(RefCell::new(create_canvas("skia", 750,800/*self.width, self.height*/ ).unwrap())));
        let canvas = Rc::new(RefCell::new(create_canvas("skia", 750,800/*self.width, self.height*/ ).unwrap()));
        let drawer = Rc::new(RefCell::new(create_drawer("std", Rc::clone(canvas.borrow())).unwrap()));
        // self.set_drawer(Rc::new(RefCell::new(create_drawer("std", Rc::clone(self.canvas.as_ref().unwrap().borrow())).unwrap())));
        //self.drawer.as_ref().unwrap().borrow_mut().fill([255, 0, 0, 255]);
        drawer.borrow_mut().draw_line(100, 100, 400, 400, [0, 255, 0, 255]);
        // ui.on_request_increase_value(move || {
        //     let ui = ui_handle.unwrap();
        //     ui.set_counter(ui.get_counter() + 1);
        // });


        // let image = Image::from_rgba8_premultiplied(pixel_buffer);

        // self.drawer.as_ref().unwrap().borrow_mut().draw_line(300, 100, 400, 400, [0, 255, 0, 255]);
        // pixmap.data_mut().copy_from_slice(self.drawer.as_ref().unwrap().borrow_mut().get_frame().as_slice());
        // ui.set_canvas(image);

        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(750, 800);
        let width = pixel_buffer.width();
        let height = pixel_buffer.height();
        let mut pixmap = tiny_skia::PixmapMut::from_bytes(
            pixel_buffer.make_mut_bytes(), width, height
        ).unwrap();
        pixmap.fill(tiny_skia::Color::BLACK);
        ui.set_canvas(Image::from_rgba8_premultiplied(pixel_buffer));

        let ui_handle2 = ui.as_weak();
        ui.on_draw_line(move || {

            let mut draww = drawer.borrow_mut();
            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(750, 800);
            let width = pixel_buffer.width();
            let height = pixel_buffer.height();
            let mut pixmap = tiny_skia::PixmapMut::from_bytes(
                pixel_buffer.make_mut_bytes(), width, height
            ).unwrap();
            draww.fill([0, 0, 0, 255]);
            pixmap.fill(tiny_skia::Color::BLACK);

            let ui = ui_handle2.unwrap();
            draww.draw_line(100, 100, 400, 400, [0, 255, 0, 255]);
            pixmap.data_mut().copy_from_slice(draww.get_frame().as_slice());
            println!("bababa");
            let image =
                Image::from_rgba8_premultiplied(pixel_buffer);
            ui.set_canvas(image.clone());
            let mut load_manager = LoadManager::new();
            let model = load_manager.load("./model/cube.txt", "frame").unwrap();
            // let figure = model.get_model().borrow_mut();
            // println!("{:?}", model.get_model().borrow_mut().get_model().get_points_mut());
            // image
            // Image::from_rgba8_premultiplied(pixel_buffer)
            });
        ui.run();
    }
}
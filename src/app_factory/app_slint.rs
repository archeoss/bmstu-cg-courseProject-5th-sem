slint::include_modules!();
use std::cell::RefCell;
use std::rc::Rc;

// use std::sync::{Arc, Mutex};
use crate::app_factory::app::MainApp;
use crate::app_factory::canvas_factory::canvas::Canvas;
use crate::app_factory::canvas_factory::create_canvas;
use crate::app_factory::drawer::{create_frame_drawer, FrameDrawer};
use crate::errors::focus_error::FocusErr;
use crate::managers::load_manager::LoadManager;
use crate::managers::transform_manager::TransformManager;
use crate::models::frame_model::FrameFigure;
use crate::models::model::Model;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString};

pub struct SlintApp
{
    width: u32,
    height: u32,
    canvas: Rc<RefCell<Box<dyn Canvas>>>,
    drawer: Rc<RefCell<Box<dyn FrameDrawer>>>,
    models: Rc<RefCell<Vec<Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>>>>,
    load_manager: Rc<RefCell<Box<LoadManager>>>,
    transform_manager: Rc<RefCell<Box<TransformManager>>>,
}

type Params = (f32, f32, f32);
impl SlintApp
{
    // pub fn show_error_dialog(ui: ,error_str: SharedString) {
    //     error_dialog.set_error_text(error_str);
    //     error_dialog.show();
    // }

    fn set_canvas(&mut self, canvas: Rc<RefCell<Box<dyn Canvas>>>)
    {
        self.canvas = canvas;
    }
    fn set_drawer(&mut self, drawer: Rc<RefCell<Box<dyn FrameDrawer>>>)
    {
        self.drawer = drawer;
    }

    fn load_model(&mut self, path: &str, model_type: &str)
        -> Result<(), Box<dyn std::error::Error>>
    {
        let load_manager = self.load_manager.as_ref();
        let model = load_manager.borrow_mut().load(path, model_type);
        match model {
            Ok(model) => {
                let model = Rc::new(RefCell::new(model));
                self.add_model(model);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn transform_model(
        &mut self,
        mut focus: usize,
        mov: Params,
        rot: Params,
        scale: Params,
    ) -> Result<(), Box<dyn std::error::Error>>
    {
        let models = self.models.clone();
        let len = models.borrow().len();
        if focus > len || focus == 0 {
            return Err(Box::new(FocusErr::new(
                "transform_model",
                len as isize,
                focus as isize,
            )));
        }
        focus -= 1;
        let mut transform_manager = self.transform_manager.as_ref().borrow_mut();
        transform_manager.move_model(models.borrow_mut()[focus].clone(), mov.0, mov.1, mov.2);
        transform_manager.rotate_model(models.borrow_mut()[focus].clone(), rot.0, rot.1, rot.2);
        transform_manager.scale_model(
            models.borrow_mut()[focus].clone(),
            scale.0,
            scale.1,
            scale.2,
        );
        Ok(())
    }

    fn update(&mut self) -> SharedPixelBuffer<Rgba8Pixel>
    {
        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(self.width, self.height);

        let width = pixel_buffer.width();
        let height = pixel_buffer.height();

        let on_draw_models = self.models.clone();
        let drawer = self.drawer.clone();

        let mut pixmap =
            tiny_skia::PixmapMut::from_bytes(pixel_buffer.make_mut_bytes(), width, height).unwrap();
        // pixmap.fill(tiny_skia::Color::BLACK);
        drawer.borrow_mut().fill([0, 0, 0, 255]);
        let i = on_draw_models.borrow_mut().len() - 1;
        drawer
            .borrow_mut()
            .draw_frame_model(on_draw_models.borrow_mut()[i].clone());
        pixmap
            .data_mut()
            .copy_from_slice(drawer.borrow_mut().get_frame().as_slice());

        pixel_buffer
    }

    fn add_model(&mut self, model: Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>)
    {
        self.models.borrow_mut().push(model);
    }
}

impl MainApp for SlintApp
{
    fn new(width: u32, height: u32) -> Box<dyn MainApp>
    {
        let canvas = Rc::new(RefCell::new(
            create_canvas("skia", width, height /*self.width, self.height*/).unwrap(),
        ));
        let drawer = Rc::new(RefCell::new(
            create_frame_drawer("std", canvas.clone()).unwrap(),
        ));

        Box::new(Self {
            width,
            height,
            // canvas: None,
            // drawer: None
            canvas, //: Some(canvas),
            drawer, //: Some(drawer)
            models: Rc::new(RefCell::new(Vec::new())),
            load_manager: Rc::new(RefCell::new(Box::new(LoadManager))),
            transform_manager: Rc::new(RefCell::new(Box::new(TransformManager))),
        })
    }

    fn launch(&'static mut self)
    {
        let ui = AppWindow::new();
        let _ui_handle = ui.as_weak();
        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(self.width, self.height);
        let width = pixel_buffer.width();
        let height = pixel_buffer.height();
        let mut pixmap =
            tiny_skia::PixmapMut::from_bytes(pixel_buffer.make_mut_bytes(), width, height).unwrap();
        pixmap.fill(tiny_skia::Color::BLACK);
        ui.set_canvas(Image::from_rgba8_premultiplied(pixel_buffer));

        let _ui_handle = ui.as_weak();
        let s = Rc::new(RefCell::new(self));

        let ui_handle = ui.as_weak();
        ui.on_run_command(move || {
            let ui = ui_handle.unwrap();
            let command = ui.get_command();
            let error_str;
            match command.as_str() {
                "draw-frame-model" => {
                    let path = std::env::current_dir().unwrap();
                    let task = rfd::FileDialog::new().set_directory(path).pick_file();
                    match task {
                        Some(path) => {
                            let model = s
                                .clone()
                                .borrow_mut()
                                .load_model(path.to_str().unwrap(), "frame");
                            match model {
                                Ok(_) => {
                                    error_str = SharedString::from("Model loaded");
                                    let image = Image::from_rgba8_premultiplied(
                                        s.clone().borrow_mut().update(),
                                    );
                                    ui.set_canvas(image);
                                }
                                Err(e) => {
                                    error_str = SharedString::from(format!("Error: {e}"));
                                }
                            }
                        }
                        None => {
                            error_str = SharedString::from("Incorrect path");
                        }
                    }
                }
                "move-model" => {
                    match s.clone().borrow_mut().transform_model(
                        0,
                        (100.0, 0.0, 0.0),
                        (0.0, 0.0, 0.0),
                        (1.0, 1.0, 1.0),
                    ) {
                        Ok(_) => {
                            error_str = SharedString::from("Model moved");
                            let image =
                                Image::from_rgba8_premultiplied(s.clone().borrow_mut().update());
                            ui.set_canvas(image);
                        }
                        Err(e) => {
                            error_str = SharedString::from(format!("Error: {e}"));
                        }
                    }
                }
                "rotate-model" => {
                    match s.clone().borrow_mut().transform_model(
                        0,
                        (0.0, 0.0, 0.0),
                        (30.0, 30.0, 30.0),
                        (1.0, 1.0, 1.0),
                    ) {
                        Ok(_) => {
                            error_str = SharedString::from("Model rotated");
                            let image =
                                Image::from_rgba8_premultiplied(s.clone().borrow_mut().update());
                            ui.set_canvas(image);
                        }
                        Err(e) => {
                            error_str = SharedString::from(format!("Error: {e}"));
                        }
                    }
                }
                "scale-model" => {
                    match s.clone().borrow_mut().transform_model(
                        0,
                        (0.0, 0.0, 0.0),
                        (0.0, 0.0, 0.0),
                        (2.0, 2.0, 2.0),
                    ) {
                        Ok(_) => {
                            error_str = SharedString::from("Model scaled");
                            let image =
                                Image::from_rgba8_premultiplied(s.clone().borrow_mut().update());
                            ui.set_canvas(image);
                        }
                        Err(e) => {
                            error_str = SharedString::from(format!("Error: {e}"));
                        }
                    }
                }
                _ => {
                    error_str = SharedString::from("Unknown command");
                }
            }
            ui.set_error_text(error_str);
            ui.set_err_visible(true);
        });

        ui.run();
    }
}

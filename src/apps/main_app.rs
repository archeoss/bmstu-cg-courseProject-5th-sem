use crate::app_factory::canvas_factory::canvas::Canvas;
use crate::app_factory::canvas_factory::create_canvas;
use crate::app_factory::drawer::{create_frame_drawer, FrameDrawer};
use crate::constants;
use crate::custom_loader::builder_factory::Builder;
use crate::managers::camera_manager::CameraManager;
use crate::managers::load_manager::LoadManager;
use crate::managers::transform_manager::TransformManager;
use crate::models::frame_model::{FrameFigure, FrameModel, Point};
use crate::models::model::{self, Model};
use crate::objects::camera;
use crate::objects::revolution::RevolutionBuilder;

use crate::objects::revolution::cone::Cone;
use crate::objects::revolution::cylinder::Cylinder;
use crate::objects::revolution::sphere::Sphere;
use crate::objects::revolution::BodiesOfRevolution;
use egui::{
    containers::{CollapsingHeader, Frame},
    widgets::Widget,
    Color32, Painter, Shape, Stroke, Ui,
};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::fmt::Pointer;
use std::path::{PathBuf, StripPrefixError};
use std::rc::Rc;
// use egui_extras::RetainedImage;
// use glium::texture::pixel_buffer::PixelBuffer;
use egui::{ColorImage, Event, Key, Mesh, Pos2, Rect};
// #[derive(PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(default))]
pub struct MainApp {
    paused: bool,
    time: f64,
    show_frame: bool,
    zoom: f32,
    start_line_width: f32,
    width_factor: f32,
    line_count: usize,
    canvas: Rc<RefCell<Box<dyn Canvas>>>,
    drawer: Rc<RefCell<Box<dyn FrameDrawer>>>,
    models: Rc<RefCell<Vec<Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>>>>,
    load_manager: Rc<RefCell<Box<LoadManager>>>,
    transform_manager: Rc<RefCell<Box<TransformManager>>>,
    camera_manager: Rc<RefCell<Box<CameraManager>>>,
    error_window: bool,
    error_str: String,
    mv: (f64, f64, f64),
    rot: (f64, f64, f64),
    scl: (f64, f64, f64),
    fov: f64,
    far: f64,
    near: f64,
    color: [u8; 4],
    light: Point<f64>,
    mv_light: (f64, f64, f64),
    to_modify: Option<(Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>, usize)>,
    window_modify: bool,
    color_to_modify: [u8; 4],
    param1_modify: f64,
    param2_modify: f64,
    param3_modify: f64,
    param1_int_modify: i64,
    param2_int_modify: i64,
    param3_int_modify: i64,
}

impl Default for MainApp {
    fn default() -> Self {
        let canvas = Rc::new(RefCell::new(
            create_canvas("skia", 3200, 2600 /*self.width, self.height*/).unwrap(),
        ));
        let drawer = Rc::new(RefCell::new(
            create_frame_drawer("std", canvas.clone()).unwrap(),
        ));
        let (width, height) = (
            canvas.borrow().width() as f64,
            canvas.borrow().height() as f64,
        );
        Self {
            paused: false,
            show_frame: false,
            time: 0.0,
            zoom: 0.25,
            start_line_width: 2.5,
            width_factor: 0.9,
            line_count: 0,
            canvas, //: Some(canvas),
            drawer, //: Some(drawer)
            models: Rc::new(RefCell::new(Vec::new())),
            load_manager: Rc::new(RefCell::new(Box::new(LoadManager))),
            transform_manager: Rc::new(RefCell::new(Box::new(TransformManager::new()))),
            camera_manager: Rc::new(RefCell::new(Box::new(CameraManager::new(
                (PI / 30.0).to_radians(),
                1.0,
                1000.0,
                (width, height),
            )))),
            error_window: false,
            error_str: String::new(),
            mv: (0.0, 0.0, 0.0),
            rot: (0.0, 0.0, 0.0),
            scl: (1.0, 1.0, 1.0),
            fov: (PI / 30.0).to_radians(),
            far: 1000.0,
            near: 1.0,
            color: [0, 255, 255, 255],
            light: Point::new(100.0, 100.0, 100.0),
            mv_light: (0.0, 0.0, 0.0),
            to_modify: None,
            window_modify: false,
            color_to_modify: [0, 0, 0, 0],
            param1_modify: 0.0,
            param2_modify: 0.0,
            param3_modify: 0.0,
            param1_int_modify: 0,
            param2_int_modify: 0,
            param3_int_modify: 0,
        }
    }
}

impl MainApp {
    fn load_model(
        &mut self,
        path: &str,
        model_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let load_manager = self.load_manager.as_ref();
        let model = load_manager.borrow_mut().load(path, model_type, self.color);
        match model {
            Ok(model) => {
                let model = Rc::new(RefCell::new(model));
                self.add_model(model);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn add_model(&mut self, model: Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>) {
        self.models.borrow_mut().push(model);
        self.transform_manager.borrow_mut().expand();
    }

    // fn delete_model(&mut self, index: usize)
    // {
    //     self.models.borrow_mut().remove(index);
    //     self.transform_manager.borrow_mut().expand();
    // }

    pub fn ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>) {
        if !self.paused {
            self.time = seconds_since_midnight.unwrap_or_else(|| ui.input().time);
            ui.ctx().request_repaint();
        }
        if self.window_modify {
            if self.to_modify.is_none() {
                self.window_modify = false;
            } else {
                let mut model = self.to_modify.as_ref().unwrap().clone();
                let string = self.error_str.clone();
                let mut tmp = true;
                egui::Window::new("Modify").collapsible(false).show(
                    ui.ctx(),
                    |ui| {
                        self.modify_ui(ui, model.0.clone(), model.1);
                    }, // if ui.button("Modify").clicked() {
                       //
                       // }
                );
            }
        }
        if self.error_window {
            let string = self.error_str.clone();
            egui::Window::new("Result")
                .open(&mut self.error_window)
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.label(string);
                });
        }

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );
        self.paint(&painter);
        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());

        Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Settings")
                    .show(ui, |ui| self.options_ui(ui, seconds_since_midnight));
            });
        Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Models").show(ui, |ui| self.models_ui(ui));
            });
        Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Cameras").show(ui, |ui| self.cameras_ui(ui));
            });
        Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Revolution").show(ui, |ui| self.revolutions_ui(ui));
            });
        Frame::popup(ui.style())
            .stroke(Stroke::NONE)
            .show(ui, |ui| {
                ui.set_max_width(270.0);
                CollapsingHeader::new("Misc.").show(ui, |ui| self.misc(ui));
            });
        self.event_handler(ui);
    }

    fn modify_ui(
        &mut self,
        ui: &mut Ui,
        model: Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>,
        ind: usize,
    ) {
        let mut builder = RevolutionBuilder::new();
        let transform = model.borrow().transform();
        let bind = model.borrow();
        let name = bind.name().clone();
        let name = name.as_str();
        match name {
            "Sphere" => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("New Radius");
                        egui::DragValue::new(&mut self.param1_modify)
                            .clamp_range(10.0..=500.0)
                            .speed(0.5)
                            .suffix(" px")
                            .ui(ui)
                            .on_hover_text("Radius of sphere");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Subdiv.");
                        egui::DragValue::new(&mut self.param1_int_modify)
                            .clamp_range(0..=10)
                            .speed(1)
                            .suffix("")
                            .ui(ui)
                            .on_hover_text("Move along y axis");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Color");
                        ui.color_edit_button_srgba_premultiplied(&mut self.color_to_modify);
                    });
                    if ui.button("Modify").clicked() {
                        let model = builder.build(
                            String::from("Sphere"),
                            BodiesOfRevolution::Sphere(
                                self.param1_modify,
                                self.param1_int_modify as usize,
                            ),
                            self.color_to_modify,
                        );
                        match model {
                            Ok(model) => {
                                let model = Rc::new(RefCell::new(model));
                                model.borrow_mut().transform_self(transform);
                                self.models.borrow_mut()[ind] = model;
                            }
                            Err(e) => {
                                self.error_window = true;
                                self.error_str = format!("Error: {e}");
                            }
                        }
                    }
                });
            }
            "Cone" => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("New Radius");
                        egui::DragValue::new(&mut self.param1_modify)
                            .clamp_range(10.0..=500.0)
                            .speed(0.5)
                            .suffix(" px")
                            .ui(ui)
                            .on_hover_text("Radius of sphere");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Height");
                        egui::DragValue::new(&mut self.param2_modify)
                            .clamp_range(10.0..=500.0)
                            .speed(0.5)
                            .suffix(" px")
                            .ui(ui)
                            .on_hover_text("Radius of sphere");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Subdiv.");
                        egui::DragValue::new(&mut self.param1_int_modify)
                            .clamp_range(0..=10)
                            .speed(1)
                            .suffix("")
                            .ui(ui)
                            .on_hover_text("Move along y axis");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Color");
                        ui.color_edit_button_srgba_premultiplied(&mut self.color_to_modify);
                    });
                    if ui.button("Modify").clicked() {
                        let model = builder.build(
                            String::from("Cone"),
                            BodiesOfRevolution::Cone(
                                self.param1_modify,
                                self.param2_modify,
                                self.param1_int_modify as usize,
                            ),
                            self.color_to_modify,
                        );
                        match model {
                            Ok(model) => {
                                let model = Rc::new(RefCell::new(model));
                                model.borrow_mut().transform_self(transform);

                                self.models.borrow_mut()[ind] = model;
                            }
                            Err(e) => {
                                self.error_window = true;
                                self.error_str = format!("Error: {e}");
                            }
                        }
                    }
                });
            }
            "Cylinder" => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("New Radius");
                        egui::DragValue::new(&mut self.param1_modify)
                            .clamp_range(10.0..=500.0)
                            .speed(0.5)
                            .suffix(" px")
                            .ui(ui)
                            .on_hover_text("Radius of sphere");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Height");
                        egui::DragValue::new(&mut self.param3_modify)
                            .clamp_range(10.0..=500.0)
                            .speed(0.5)
                            .suffix(" px")
                            .ui(ui)
                            .on_hover_text("Radius of sphere");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Subdiv.");
                        egui::DragValue::new(&mut self.param1_int_modify)
                            .clamp_range(0..=10)
                            .speed(1)
                            .suffix("")
                            .ui(ui)
                            .on_hover_text("Move along y axis");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Color");
                        ui.color_edit_button_srgba_premultiplied(&mut self.color_to_modify);
                    });
                    if ui.button("Modify").clicked() {
                        let model = builder.build(
                            String::from("Cylinder"),
                            BodiesOfRevolution::Cylinder(
                                self.param1_modify,
                                self.param1_modify,
                                self.param2_modify,
                                self.param1_int_modify as usize,
                            ),
                            self.color_to_modify,
                        );
                        match model {
                            Ok(model) => {
                                let model = Rc::new(RefCell::new(model));
                                model.borrow_mut().transform_self(transform);

                                self.models.borrow_mut()[ind] = model;
                            }
                            Err(e) => {
                                self.error_window = true;
                                self.error_str = format!("Error: {e}");
                            }
                        }
                    }
                });
            }
            "Cut Cone" => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("New Lower Radius");
                        egui::DragValue::new(&mut self.param1_modify)
                            .clamp_range(10.0..=500.0)
                            .speed(0.5)
                            .suffix(" px")
                            .ui(ui)
                            .on_hover_text("Radius of sphere");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Upper Radius");
                        egui::DragValue::new(&mut self.param2_modify)
                            .clamp_range(10.0..=500.0)
                            .speed(0.5)
                            .suffix(" px")
                            .ui(ui)
                            .on_hover_text("Radius of sphere");
                    });

                    ui.horizontal(|ui| {
                        ui.label("New Height");
                        egui::DragValue::new(&mut self.param3_modify)
                            .clamp_range(10.0..=500.0)
                            .speed(0.5)
                            .suffix(" px")
                            .ui(ui)
                            .on_hover_text("Radius of sphere");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Subdiv.");
                        egui::DragValue::new(&mut self.param1_int_modify)
                            .clamp_range(0..=10)
                            .speed(1)
                            .suffix("")
                            .ui(ui)
                            .on_hover_text("Move along y axis");
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Color");
                        ui.color_edit_button_srgba_premultiplied(&mut self.color_to_modify);
                    });
                    if ui.button("Modify").clicked() {
                        let model = builder.build(
                            String::from("Cut Cone"),
                            BodiesOfRevolution::Cylinder(
                                self.param2_modify,
                                self.param1_modify,
                                self.param3_modify,
                                self.param1_int_modify as usize,
                            ),
                            self.color_to_modify,
                        );
                        match model {
                            Ok(model) => {
                                let model = Rc::new(RefCell::new(model));
                                model.borrow_mut().transform_self(transform);

                                self.models.borrow_mut()[ind] = model;
                            }
                            Err(e) => {
                                self.error_window = true;
                                self.error_str = format!("Error: {e}");
                            }
                        }
                    }
                });
            }
            _ => {
                self.error_window = true;
                self.error_str = format!("Can't modify this model");
            }
        }
        ui.separator();
        if ui.button("Close").clicked() {
            self.window_modify = false;
        }
    }
    fn misc(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(format!(
                "Light: x={} | y={} | z={}",
                self.light.x(),
                self.light.y(),
                self.light.z(),
            ));
            ui.horizontal(|ui| {
                if ui.button("Move").clicked() {
                    self.light += Point::new(self.mv_light.0, self.mv_light.1, self.mv_light.2);
                }
                egui::DragValue::new(&mut self.mv_light.0)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" px")
                    .ui(ui)
                    .on_hover_text("Move along x axis");
                egui::DragValue::new(&mut self.mv_light.1)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" px")
                    .ui(ui)
                    .on_hover_text("Move along y axis");
                egui::DragValue::new(&mut self.mv_light.2)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" px")
                    .ui(ui)
                    .on_hover_text("Move along z axis");
            });
            ui.color_edit_button_srgba_premultiplied(&mut self.color);
        });
    }

    fn event_handler(&mut self, ui: &mut Ui) {
        let events = ui.input().events.clone();
        for event in &events {
            match event {
                egui::Event::Key {
                    key,
                    pressed,
                    modifiers,
                } => {
                    if *pressed {
                        self.move_camera(*key);
                    }
                }
                egui::Event::Text(t) => {
                    // println!("Text = {:?}", t);
                }
                _ => {}
            }
        }
    }

    fn move_camera(&mut self, key: Key) {
        let active_cam = self.camera_manager.borrow_mut().active_camera().unwrap();
        let mut active_cam = active_cam.borrow_mut();
        let speed = constants::BASE_CAMERA_SPEED * constants::CAMERA_MULTIPLIER_SPEED;
        let rot_speed = speed.to_radians();
        match key {
            Key::ArrowLeft | Key::A => active_cam.move_right(-speed),
            Key::ArrowUp => active_cam.move_up(speed),
            Key::ArrowRight | Key::D => active_cam.move_right(speed),
            Key::ArrowDown => active_cam.move_up(-speed),
            Key::W => active_cam.move_forward(speed),
            Key::S => active_cam.move_forward(-speed),
            Key::E => active_cam.pitch(rot_speed),
            Key::Q => active_cam.pitch(-rot_speed),
            Key::Z => active_cam.yaw(rot_speed),
            Key::C => active_cam.yaw(-rot_speed),
            Key::R => active_cam.roll(rot_speed),
            Key::T => active_cam.roll(-rot_speed),
            // Key::E => active_cam.pitch(speed),
            // Key::Q => active_cam.pitch(-speed),
            _ => {}
        }
    }

    fn revolutions_ui(&mut self, ui: &mut Ui) {
        let mut builder = RevolutionBuilder::new();
        ui.label("Bodies of revolution");
        if ui.button("Sphere").clicked() {
            let model = builder.build(
                String::from("Sphere"),
                BodiesOfRevolution::Sphere(60.0, 1),
                self.color,
            );
            match model {
                Ok(model) => {
                    let model = Rc::new(RefCell::new(model));
                    self.add_model(model);
                }
                Err(e) => {
                    self.error_window = true;
                    self.error_str = format!("Error: {e}");
                }
            }
        }
        if ui.button("Cone").clicked() {
            let model = builder.build(
                String::from("Cone"),
                BodiesOfRevolution::Cone(60.0, 60.0, 1),
                self.color,
            );
            match model {
                Ok(model) => {
                    let model = Rc::new(RefCell::new(model));
                    self.add_model(model);
                }
                Err(e) => {
                    self.error_window = true;
                    self.error_str = format!("Error: {e}");
                }
            }
        }
        if ui.button("Cylinder").clicked() {
            let model = builder.build(
                String::from("Cylinder"),
                BodiesOfRevolution::Cylinder(60.0, 60.0, 60.0, 1),
                self.color,
            );
            match model {
                Ok(model) => {
                    let model = Rc::new(RefCell::new(model));
                    self.add_model(model);
                }
                Err(e) => {
                    self.error_window = true;
                    self.error_str = format!("Error: {e}");
                }
            }
        }
        if ui.button("Cut Cone").clicked() {
            let model = builder.build(
                String::from("Cut Cone"),
                BodiesOfRevolution::Cylinder(40.0, 70.0, 60.0, 1),
                self.color,
            );
            match model {
                Ok(model) => {
                    let model = Rc::new(RefCell::new(model));
                    self.add_model(model);
                }
                Err(e) => {
                    self.error_window = true;
                    self.error_str = format!("Error: {e}");
                }
            }
        }
    }

    fn set_modifiers(&mut self, model: Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>) {
        let bind = model.borrow();
        let name = bind.name().clone();
        let name = name.as_str();
        // drop(bind);
        match name {
            "Sphere" => {
                let model = bind.as_any().downcast_ref::<Sphere>().unwrap();
                self.param1_modify = model.radius();
                self.param1_int_modify = model.subdivision() as i64;
                self.color_to_modify = model.figures()[0].borrow().figures()[0].borrow().color();
            }
            "Cone" => {
                let model = bind.as_any().downcast_ref::<Cone>().unwrap();
                self.param1_modify = model.radius();
                self.param2_modify = model.height();
                self.param1_int_modify = model.subdivision() as i64;
                self.color_to_modify = model.figures()[0].borrow().figures()[0].borrow().color();
            }
            _ => {}
        }
    }

    fn models_ui(&mut self, ui: &mut Ui) {
        ui.label("Models to transform");
        let len = self.models.borrow().len();
        let mut models = self.models.borrow_mut();
        let mut removed = 0; // костыль
        for i in 0..len {
            ui.horizontal(|ui| {
                let model = models[i - removed].borrow();
                let model_center = models[i - removed].borrow().center();
                ui.checkbox(
                    &mut self.transform_manager.borrow_mut().to_transform()[i - removed],
                    model.name(),
                );

                if ui.button("Modify").clicked() {
                    // self.delete_model(i);
                    let bind = models[i - removed].borrow();
                    let name = bind.name().clone();
                    let name = name.as_str();
                    // drop(bind);
                    match name {
                        "Sphere" => {
                            let model = bind.as_any().downcast_ref::<Sphere>().unwrap();
                            self.param1_modify = model.radius();
                            self.param1_int_modify = model.subdivision() as i64;
                            self.color_to_modify =
                                model.figures()[0].borrow().figures()[0].borrow().color();
                        }
                        "Cone" => {
                            let model = bind.as_any().downcast_ref::<Cone>().unwrap();
                            self.param1_modify = model.radius();
                            self.param2_modify = model.height();
                            self.param1_int_modify = model.subdivision() as i64;
                            self.color_to_modify =
                                model.figures()[0].borrow().figures()[0].borrow().color();
                        }
                        "Cut Cone" | "Cylinder" => {
                            let model = bind.as_any().downcast_ref::<Cylinder>().unwrap();
                            self.param1_modify = model.lower_radius();
                            self.param2_modify = model.upper_radius();
                            self.param3_modify = model.height();
                            self.param1_int_modify = model.subdivision() as i64;
                            self.color_to_modify =
                                model.figures()[0].borrow().figures()[0].borrow().color();
                        }
                        _ => {}
                    }
                    self.to_modify = Some((models[i - removed].clone(), i - removed));
                    self.window_modify = true;
                }
                // Delete
                drop(model);
                if ui.button("Delete").clicked() {
                    // self.delete_model(i);
                    models.remove(i - removed);
                    self.transform_manager.borrow_mut().remove(i - removed);
                    removed += 1;
                }

                ui.label(format!("pos: {:.2?}", model_center.xyz()));
            });
        }
        drop(models);
        ui.separator();
        ui.label("Translation");

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Move").clicked() {
                    let mut models = self.models.borrow_mut().clone();
                    let mut mngr = self.transform_manager.borrow_mut();
                    let to_transform = mngr.to_transform();
                    let mut iter = to_transform.iter();
                    models.retain(|_| *iter.next().unwrap());
                    mngr.move_models(&mut models, self.mv);
                }
                egui::DragValue::new(&mut self.mv.0)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" px")
                    .ui(ui)
                    .on_hover_text("Move along x axis");
                egui::DragValue::new(&mut self.mv.1)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" px")
                    .ui(ui)
                    .on_hover_text("Move along y axis");
                egui::DragValue::new(&mut self.mv.2)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" px")
                    .ui(ui)
                    .on_hover_text("Move along z axis");
            });
            ui.horizontal(|ui| {
                if ui.button("Rotate").clicked() {
                    let mut models = self.models.borrow_mut().clone();
                    let mut mngr = self.transform_manager.borrow_mut();
                    let to_transform = mngr.to_transform();
                    let mut iter = to_transform.iter();
                    models.retain(|_| *iter.next().unwrap());
                    mngr.rotate_models(&mut models, self.rot);
                }
                egui::DragValue::new(&mut self.rot.0)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" deg")
                    .ui(ui)
                    .on_hover_text("Rotate along x axis");
                egui::DragValue::new(&mut self.rot.1)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" deg")
                    .ui(ui)
                    .on_hover_text("Rotate along y axis");
                egui::DragValue::new(&mut self.rot.2)
                    .clamp_range(-500.0..=500.0)
                    .speed(0.5)
                    .suffix(" deg")
                    .ui(ui)
                    .on_hover_text("Rotate along z axis");
            });

            ui.horizontal(|ui| {
                if ui.button("Scale").clicked() {
                    let mut models = self.models.borrow_mut().clone();
                    let mut mngr = self.transform_manager.borrow_mut();
                    let to_transform = mngr.to_transform();
                    let mut iter = to_transform.iter();
                    models.retain(|_| *iter.next().unwrap());
                    mngr.scale_models(&mut models, self.scl);
                }
                egui::DragValue::new(&mut self.scl.0)
                    .clamp_range(-10.0..=10.0)
                    .speed(0.1)
                    .suffix(" k")
                    .ui(ui)
                    .on_hover_text("Scale along x axis");
                egui::DragValue::new(&mut self.scl.1)
                    .clamp_range(-10.0..=10.0)
                    .speed(0.1)
                    .suffix(" k")
                    .ui(ui)
                    .on_hover_text("Scale along y axis");
                egui::DragValue::new(&mut self.scl.2)
                    .clamp_range(-10.0..=10.0)
                    .speed(0.1)
                    .suffix(" k")
                    .ui(ui)
                    .on_hover_text("Scale along z axis");
            });
            ui.horizontal(|ui| {
                if ui.button("Merge").clicked() {
                    let mut models = self.models.borrow_mut().clone();
                    let mut mngr = self.transform_manager.borrow_mut();
                    let to_transform = mngr.to_transform();
                    let mut iter = to_transform.iter();
                    models.retain(|_| *iter.next().unwrap());
                    for model in models.iter().skip(1) {
                        for figure in model.borrow().figures() {
                            models[0].borrow_mut().add_figure(figure);
                        }
                    }
                    models[0].borrow_mut().set_name("Abstract");
                    let to_transform: Vec<bool> = to_transform.iter().map(|x| *x).collect();
                    drop(mngr);
                    drop(models);
                    let mut flag = false;
                    for (i, del) in to_transform.iter().enumerate() {
                        if *del && !flag {
                            flag = !flag;
                        } else if *del && flag {
                            self.models.borrow_mut().remove(i - removed);
                            self.transform_manager.borrow_mut().remove(i - removed);
                            removed += 1;
                        }
                    }
                }
            });
        });
    }

    fn cameras_ui(&mut self, ui: &mut Ui) {
        dbg!((
            self.canvas.borrow().width() as f64,
            self.canvas.borrow().height() as f64,
        ));
        ui.label("Cameras");
        let len = self.camera_manager.borrow_mut().amount();
        for i in 0..len {
            ui.horizontal(|ui| {
                ui.label(format!("Cam #{i}"));
                if ui.button("Select").clicked() {
                    self.camera_manager.borrow_mut().set_active_camera(i);
                    self.camera_manager.borrow_mut().update_aspect((
                        self.canvas.borrow().width() as f64,
                        self.canvas.borrow().height() as f64,
                    ));
                }
                if ui.button("Clone").clicked() {
                    let cam = self.camera_manager.borrow().camera(i).unwrap();
                    self.camera_manager.borrow_mut().add_camera(cam);
                }
                if ui.button("Delete").clicked() {
                    if self.camera_manager.borrow().amount() == 1 {
                        self.error_window = true;
                        self.error_str = String::from("Can't delete last camera");
                    } else {
                        self.camera_manager.borrow_mut().delete_camera(i);
                    }
                }
            });
        }
        ui.separator();
        ui.label("Spawn");
        if ui.button("Spawn").clicked() {
            // self.delete_model(i);
            self.camera_manager.borrow_mut().spawn_camera();
            // let amount = self.camera_manager.borrow_mut().amount();
            // let camera = self.camera_manager.borrow_mut().camera(amount - 1);
            // camera.move_
        }
        ui.separator();
        egui::Slider::new(
            &mut self.fov,
            (PI / 40.0).to_radians()..=(PI / 20.0).to_radians(),
            // 1.0..=50.00,
        )
        .show_value(false)
        .clamp_to_range(true)
        .text("FOV Y")
        .ui(ui);
        egui::DragValue::new(&mut self.near)
            .clamp_range(0.0..=1000.0)
            .speed(5.0)
            .ui(ui)
            .on_hover_text("Near");
        egui::DragValue::new(&mut self.far)
            .clamp_range(10.0..=10000.0)
            .speed(5.0)
            .ui(ui)
            .on_hover_text("Far");
        if self.near >= self.far {
            self.near = self.far - 1.0;
        }
        ui.separator();
        self.camera_manager.borrow_mut().update_fov(self.fov);
        self.camera_manager.borrow_mut().update_near(self.near);
        self.camera_manager.borrow_mut().update_far(self.far);
        let camera = self.camera_manager.borrow().active_camera();
        let xyz = camera.as_ref().unwrap().borrow().pos();
        ui.label(format!(
            "pos: x = {:.2}, y = {:.2}, z = {:.2}",
            xyz.x, xyz.y, xyz.z
        ));
        let xyz = &camera.as_ref().unwrap().borrow().target();
        ui.label(format!(
            "target: x = {:.2}, y = {:.2}, z = {:.2}",
            xyz.x, xyz.y, xyz.z
        ));
        let xyz = &camera.as_ref().unwrap().borrow().up();
        ui.label(format!(
            "up: x = {:.2}, y = {:.2}, z = {:.2}",
            xyz.x, xyz.y, xyz.z
        ));
        dbg!(camera);
    }

    fn options_ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>) {
        if seconds_since_midnight.is_some() {
            ui.label(format!(
                "Local time: {:02}:{:02}:{:02}.{:03}",
                (self.time % (24.0 * 60.0 * 60.0) / 3600.0).floor(),
                (self.time % (60.0 * 60.0) / 60.0).floor(),
                (self.time % 60.0).floor(),
                (self.time % 1.0 * 100.0).floor()
            ));
        } else {
            ui.label("The fractal_clock clock is not showing the correct time");
        };
        ui.label(format!("Painted line count: {}", self.line_count));

        ui.checkbox(&mut self.paused, "Paused");
        ui.checkbox(&mut self.show_frame, "Show frame");
        ui.separator();
        #[cfg(target_arch = "wasm32")]
        {
            ui.label("File loader disabled in Web");
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Load Model").clicked() {
            let path = std::env::current_dir().unwrap();
            let task = rfd::FileDialog::new().set_directory(path).pick_file();
            match task {
                Some(path) => {
                    let model = self.load_model(path.to_str().unwrap(), "frame");
                    match model {
                        Ok(_) => {
                            self.error_window = true;
                            self.error_str = String::from("Model loaded");
                        }
                        Err(e) => {
                            self.error_window = true;
                            self.error_str = format!("Error: {e}");
                        }
                    }
                }
                None => {
                    self.error_window = true;
                    self.error_str = String::from("Incorrect path");
                }
            }
        }

        // ui.add(Slider::new(&mut self.zoom, 0.0..=1.0).text("zoom"));
        // ui.add(Slider::new(&mut self.start_line_width, 0.0..=5.0).text("Start line width"));
        // ui.add(Slider::new(&mut self.depth, 0..=14).text("depth"));
        // ui.add(Slider::new(&mut self.length_factor, 0.0..=1.0).text("length factor"));
        // ui.add(Slider::new(&mut self.luminance_factor, 0.0..=1.0).text("luminance factor"));
        // ui.add(Slider::new(&mut self.width_factor, 0.0..=1.0).text("width factor"));
        //
        // // egui::reset_button(ui, self);
        //
    }

    fn paint(&mut self, painter: &Painter) {
        // let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(self.width, self.height);
        let rect = painter.clip_rect();
        let width = rect.width() as usize;
        let height = rect.height() as usize;
        let active_cam = self.camera_manager.borrow().active_camera();
        let on_draw_models = self.models.clone();
        let canvas = Rc::new(RefCell::new(
            create_canvas(
                "skia",
                width.try_into().unwrap(),
                height.try_into().unwrap(), /*self.width, self.height*/
            )
            .unwrap(),
        ));
        let drawer = Rc::new(RefCell::new(create_frame_drawer("std", canvas).unwrap()));
        drawer.borrow_mut().set_camera(active_cam.unwrap());
        drawer.borrow_mut().fill([200, 200, 200, 255]);
        let models = on_draw_models.borrow_mut();
        drawer
            .borrow_mut()
            .draw_in_3d(models.as_slice(), self.light);
        if self.show_frame {
            drawer.borrow_mut().draw_frame_model(models.as_slice());
        }
        let image = ColorImage::from_rgba_unmultiplied(
            [width, height],
            drawer.borrow_mut().frame().as_slice(),
        );
        let texture = painter
            .ctx()
            .load_texture("Canvas", image, Default::default());
        let mut mesh = Mesh::with_texture(texture.id());
        mesh.add_rect_with_uv(
            rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
        painter.add(Shape::mesh(mesh));
    }
}

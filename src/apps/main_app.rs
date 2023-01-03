use crate::app_factory::canvas_factory::canvas::Canvas;
use crate::app_factory::canvas_factory::create_canvas;
use crate::app_factory::drawer::{create_frame_drawer, FrameDrawer};
use crate::managers::load_manager::LoadManager;
use crate::managers::transform_manager::TransformManager;
use crate::models::frame_model::FrameFigure;
use crate::models::model::Model;
use egui::{containers::*, widgets::*, *};
use std::cell::RefCell;
use std::rc::Rc;
// use egui_extras::RetainedImage;
// use glium::texture::pixel_buffer::PixelBuffer;
use egui::{ColorImage, Mesh, Pos2, Rect};
// #[derive(PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(default))]
pub struct MainApp
{
    paused: bool,
    time: f64,
    zoom: f32,
    start_line_width: f32,
    depth: usize,
    length_factor: f32,
    luminance_factor: f32,
    width_factor: f32,
    line_count: usize,
    canvas: Rc<RefCell<Box<dyn Canvas>>>,
    drawer: Rc<RefCell<Box<dyn FrameDrawer>>>,
    models: Rc<RefCell<Vec<Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>>>>,
    load_manager: Rc<RefCell<Box<LoadManager>>>,
    transform_manager: Rc<RefCell<Box<TransformManager>>>,
    error_window: bool,
    error_str: String,
    mv: (f64, f64, f64),
    rot: (f64, f64, f64),
    scl: (f64, f64, f64),
}

impl Default for MainApp
{
    fn default() -> Self
    {
        let canvas = Rc::new(RefCell::new(
            create_canvas("skia", 270, 450 /*self.width, self.height*/).unwrap(),
        ));
        let drawer = Rc::new(RefCell::new(
            create_frame_drawer("std", canvas.clone()).unwrap(),
        ));
        Self {
            paused: false,
            time: 0.0,
            zoom: 0.25,
            start_line_width: 2.5,
            depth: 9,
            length_factor: 0.8,
            luminance_factor: 0.8,
            width_factor: 0.9,
            line_count: 0,
            canvas, //: Some(canvas),
            drawer, //: Some(drawer)
            models: Rc::new(RefCell::new(Vec::new())),
            load_manager: Rc::new(RefCell::new(Box::new(LoadManager))),
            transform_manager: Rc::new(RefCell::new(Box::new(TransformManager::new()))),
            error_window: false,
            error_str: String::new(),
            mv: (0.0, 0.0, 0.0),
            rot: (0.0, 0.0, 0.0),
            scl: (1.0, 1.0, 1.0),
        }
    }
}

impl MainApp
{
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

    fn add_model(&mut self, model: Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>)
    {
        self.models.borrow_mut().push(model);
        self.transform_manager.borrow_mut().expand();
    }

    // fn delete_model(&mut self, index: usize)
    // {
    //     self.models.borrow_mut().remove(index);
    //     self.transform_manager.borrow_mut().expand();
    // }

    pub fn ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>)
    {
        if !self.paused {
            self.time = seconds_since_midnight.unwrap_or_else(|| ui.input().time);
            ui.ctx().request_repaint();
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
    }

    fn models_ui(&mut self, ui: &mut Ui)
    {
        ui.label("Models to transform");
        let len = self.models.borrow().len();
        let mut models = self.models.borrow_mut();
        for i in 0..len {
            ui.horizontal(|ui| {
                let model = models[i].borrow();
                ui.checkbox(
                    &mut self.transform_manager.borrow_mut().get_to_transform()[i],
                    model.get_name(),
                );
                // Delete
                drop(model);
                if ui.button("Delete").clicked() {
                    // self.delete_model(i);
                    models.remove(i);
                    self.transform_manager.borrow_mut().remove(i);
                }
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
                    let to_transform = mngr.get_to_transform();
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
                    let mut to_transform = mngr.get_to_transform();
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
                    let to_transform = mngr.get_to_transform();
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
        });
    }

    fn options_ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>)
    {
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
        ui.separator();
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
                            self.error_str = String::from(format!("Error: {e}"));
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

    fn paint(&mut self, painter: &Painter)
    {
        // let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(self.width, self.height);
        let rect = painter.clip_rect();
        let width = rect.width() as usize;
        let height = rect.height() as usize;

        let on_draw_models = self.models.clone();
        let canvas = Rc::new(RefCell::new(
            create_canvas(
                "skia",
                width.try_into().unwrap(),
                height.try_into().unwrap(), /*self.width, self.height*/
            )
            .unwrap(),
        ));
        let drawer = Rc::new(RefCell::new(
            create_frame_drawer("std", canvas.clone()).unwrap(),
        ));
        //
        // let mut pixmap =
        //     tiny_skia::PixmapMut::from_bytes(pixel_buffer.make_mut_bytes(), width, height).unwrap();
        // pixmap.fill(tiny_skia::Color::BLACK);
        // let mut pixmap: PixelBuffer<u8> = PixelBuffer::new_empty(painter.gl(), (width * height) as usize);
        drawer.borrow_mut().fill([0, 0, 0, 255]);
        let i = on_draw_models.borrow_mut().len();
        for model in on_draw_models.borrow_mut().iter_mut().take(i) {
            // let mut model = model.borrow_mut();
            drawer.borrow_mut().draw_frame_model(model.clone());
            let image = ColorImage::from_rgba_unmultiplied(
                [width, height],
                drawer.borrow_mut().get_frame().as_slice(),
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

        // texture
        // painter.add(painter.ctx().load_texture(
        //     "Canvas",
        //     image,
        //     Default::default()
        // ));
        // egui_extras::image::RetainedImage::from_image_bytes(pixmap).show(painter);
        // .show(painter);
        // pixmap
        // struct Hand {
        //     length: f32,
        //     angle: f32,
        //     vec: Vec2,
        // }
        //
        // impl Hand {
        //     fn from_length_angle(length: f32, angle: f32) -> Self {
        //         Self {
        //             length,
        //             angle,
        //             vec: length * Vec2::angled(angle),
        //         }
        //     }
        // }
        //
        // let angle_from_period =
        //     |period| TAU * (self.time.rem_euclid(period) / period) as f32 - TAU / 4.0;
        //
        // let hands = [
        //     // Second hand:
        //     Hand::from_length_angle(self.length_factor, angle_from_period(60.0)),
        //     // Minute hand:
        //     Hand::from_length_angle(self.length_factor, angle_from_period(60.0 * 60.0)),
        //     // Hour hand:
        //     Hand::from_length_angle(0.5, angle_from_period(12.0 * 60.0 * 60.0)),
        // ];
        //
        // let mut shapes: Vec<Shape> = Vec::new();
        //
        // let rect = painter.clip_rect();
        // let to_screen = emath::RectTransform::from_to(
        //     Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
        //     rect,
        // );
        //
        // let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
        //     let line = [to_screen * points[0], to_screen * points[1]];
        //
        //     // culling
        //     if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
        //         shapes.push(Shape::line_segment(line, (width, color)));
        //     }
        // };
        //
        // let hand_rotations = [
        //     hands[0].angle - hands[2].angle + TAU / 2.0,
        //     hands[1].angle - hands[2].angle + TAU / 2.0,
        // ];
        //
        // let hand_rotors = [
        //     hands[0].length * emath::Rot2::from_angle(hand_rotations[0]),
        //     hands[1].length * emath::Rot2::from_angle(hand_rotations[1]),
        // ];
        //
        // #[derive(Clone, Copy)]
        // struct Node {
        //     pos: Pos2,
        //     dir: Vec2,
        // }
        //
        // let mut nodes = Vec::new();
        //
        // let mut width = self.start_line_width;
        //
        // for (i, hand) in hands.iter().enumerate() {
        //     let center = pos2(0.0, 0.0);
        //     let end = center + hand.vec;
        //     paint_line([center, end], Color32::from_additive_luminance(255), width);
        //     if i < 2 {
        //         nodes.push(Node {
        //             pos: end,
        //             dir: hand.vec,
        //         });
        //     }
        // }
        //
        // let mut luminance = 0.7; // Start dimmer than main hands
        //
        // let mut new_nodes = Vec::new();
        // for _ in 0..self.depth {
        //     new_nodes.clear();
        //     new_nodes.reserve(nodes.len() * 2);
        //
        //     luminance *= self.luminance_factor;
        //     width *= self.width_factor;
        //
        //     let luminance_u8 = (255.0 * luminance).round() as u8;
        //     if luminance_u8 == 0 {
        //         break;
        //     }
        //
        //     for &rotor in &hand_rotors {
        //         for a in &nodes {
        //             let new_dir = rotor * a.dir;
        //             let b = Node {
        //                 pos: a.pos + new_dir,
        //                 dir: new_dir,
        //             };
        //             paint_line(
        //                 [a.pos, b.pos],
        //                 Color32::from_additive_luminance(luminance_u8),
        //                 width,
        //             );
        //             new_nodes.push(b);
        //         }
        //     }
        //
        //     std::mem::swap(&mut nodes, &mut new_nodes);
        // }
        // self.line_count = shapes.len();
        // painter.extend(shapes);
    }
}

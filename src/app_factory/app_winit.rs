use std::borrow::{BorrowMut};
use std::error::Error;
/// Representation of the application state.

use crate::app_factory::canvas::{Canvas, create_canvas};
use log::{error};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use crate::app_factory::app::App;
use async_trait::async_trait;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use pixels::{Pixels, SurfaceTexture};
use drawer::{create_drawer, Drawer};
use crate::app_factory::drawer;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BOX_SIZE: i16 = 64;

struct World
{
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

pub struct AppPixel
{
    width: u32,
    height: u32,
    canvas: Option<Arc<Mutex<Box<dyn Canvas>>>>,
    pixels: Option<Arc<Mutex<Pixels>>>,
    drawer: Option<Arc<Mutex<Box<dyn Drawer>>>>,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    fn draw(&self, frame: &mut Box<AppPixel>) {
        for i in 0..frame.width*frame.height {
            let x = (i % WIDTH) as i16;
            let y = (i / WIDTH) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            frame.drawer.as_ref().unwrap().lock().unwrap().draw_point(x as i32, y as i32, rgba);
        }
    }
}

impl AppPixel
{
    fn render(&mut self) -> Result<(), Box<dyn Error>>
    {
        self.canvas.as_ref().unwrap().lock().unwrap().copy_to_buffer(
            self.pixels.clone().unwrap().borrow_mut().lock().unwrap().get_frame());
        self.pixels.clone().unwrap().borrow_mut().lock().unwrap().render().map_err(|e| Box::from(e))
    }

    // fn get_frame(&mut self) -> &mut [u8] {
    //     self.canvas.as_mut().unwrap().lock().unwrap().get_frame()
    // }
    fn test(&mut self) {
        self.drawer.as_mut().unwrap().lock().unwrap().draw_point(100, 100, [0x5e, 0x48, 0xe8, 0xff]);
    }
    fn resize_surface(&mut self, width: u32, height: u32)
    {
        self.width = width;
        self.height = height;
        self.pixels.clone().unwrap().borrow_mut().lock().unwrap().resize_surface(width, height);
        self.canvas.as_ref().unwrap().lock().unwrap().resize_surface(width, height,
                 self.pixels.clone().unwrap().borrow_mut().lock().unwrap().get_frame());
    }
}
#[async_trait(?Send)]
impl App for AppPixel
{
    fn new(width: u32, height: u32) -> Self //where Self: 'static
    {
        Self {
            width,
            height,
            canvas: None,
            pixels: None,
            drawer: None,
        }
    }

    async fn run(&self)
    {
        panic!("Not implemented");
    }

    async fn run_wasm(mut self : Box<Self>)
    {
        let event_loop = EventLoop::new();
        let window = {
            let size =
                LogicalSize::new(self.width as f64, self.height as f64);

            WindowBuilder::new()
                .with_title("courseProject")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .expect("WindowBuilder error")
        };

        let window = Rc::new(window);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowExtWebSys;

            // Retrieve current width and height dimensions of browser client window
            let get_window_size = || {
                let client_window = web_sys::window().unwrap();
                LogicalSize::new(
                    client_window.inner_width().unwrap().as_f64().unwrap(),
                    client_window.inner_height().unwrap().as_f64().unwrap(),
                )
            };

            let window = Rc::clone(&window);

            // Initialize winit window with current dimensions of browser client
            window.set_inner_size(get_window_size());

            let client_window = web_sys::window().unwrap();

            // Attach winit canvas to body element
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");

            // Listen for resize event on browser client. Adjust winit window dimensions
            // on event trigger
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                let size = get_window_size();
                window.set_inner_size(size)
            }) as Box<dyn FnMut(_)>);
            client_window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        let mut input = WinitInputHelper::new();
        let mut pixels =
            {
                let window_size = window.inner_size();
                let surface_texture =
                    SurfaceTexture::new(window_size.width,
                                        window_size.height,
                                        window.as_ref());
                Pixels::new_async( self.width, self.height, surface_texture)
                    .await.expect("Pixels error")
            };
        let canvas =
            create_canvas("pixel", self.width, self.height,
                          pixels.get_frame()).await.expect("Canvas error");
        self.canvas = Some(Arc::new(Mutex::new(canvas)));
        self.drawer = Some(Arc::new(Mutex::new(create_drawer("std", self.canvas.as_ref().unwrap().clone()).expect("Drawer error"))));

        self.pixels = Some(Arc::new(Mutex::new(pixels)));
        let mut world = World::new();
        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                // world.draw(&mut self);
                self.test();
                if self
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e.to_string()))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Handle input events
            if input.update(&event) {
                // e events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    self.resize_surface(size.width, size.height);
                    // canvas.resize_surface(size.width, size.height, pixels.get_frame());
                }

                // Update internal state and request a redraw
                world.update();
                window.request_redraw();
            }
        });
    }
}
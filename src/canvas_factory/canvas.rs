extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::render::Renderer;

pub trait Canvas
{
    fn new(x: u32, y: u32) -> Self where Self: Sized;
    fn point(&mut self, x: i32, y: i32, color: u32);
    fn wait_for_esc(&mut self);
}

pub struct CanvasSDL
{
    renderer: Renderer<'static>,
    sdl_context: Sdl
}

impl Canvas for CanvasSDL
{
    fn new(x: u32, y: u32) -> CanvasSDL
    {
        let mut sdl_context = sdl2::init().video().unwrap();

        let window = sdl_context.window("rust-sdl2", x, y).position_centered().opengl().build().unwrap();

        let renderer = window.renderer().build().unwrap();
        CanvasSDL { renderer, sdl_context }
    }

    fn point(&mut self, x: i32, y: i32, color: u32)
    {
        self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
        self.renderer.draw_point(Point::new(x, y));
        self.renderer.present();
    }

    fn wait_for_esc(&mut self)
    {
        let mut running = true;

        while running
        {
            for event in self.sdl_context.event_pump().poll_iter()
            {
                use sdl2::event::Event as Event;

                match event
                {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false
                    },
                    _ => {}
                }
            }
        }
    }
}


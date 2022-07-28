extern crate sdl2;

use crate::canvas_factory::canvas::Canvas;

pub mod canvas_factory;
// use canvas_factory as canvas;

fn main()
{
    println!("Hello, world!");
    let mut canvas: Box<dyn Canvas> = canvas_factory::create_window("sdl").unwrap();
    canvas.point(100, 100, 0xFF0000);
    canvas.wait_for_esc();
}

use errors::wrongSizeErr::WrongSizeErr;
use std::error::Error;
use crate::app_factory::canvas::Canvas;
use async_trait::async_trait;
use crate::app_factory::errors;

pub struct CanvasPixel
{
    width: u32,
    height: u32,
    frame: Vec<u8>,
    background_color: [u8; 4]
}

#[async_trait]
impl Canvas for CanvasPixel
{
    async fn new(width: u32, height: u32, init_frame: &[u8], background_color: [u8; 4]) -> Self where Self: Sized
    {
        CanvasPixel
        {
            width,
            height,
            frame: init_frame.to_vec(),
            background_color
        }
    }
    fn point(&mut self, x: i32, y: i32, color: [u8; 4])
    {
        let i = ((x + y * self.width as i32) * 4) as usize;

        self.frame[i..i + 4].copy_from_slice(&color);
    }

    fn copy_to_buffer(&mut self, surface: &mut [u8])
    {
        surface.copy_from_slice(&self.frame);
    }

    fn get_frame(&mut self) -> &mut [u8] { &mut self.frame[..] }

    fn resize_surface(&mut self, width: u32, height: u32, new_frame: &[u8]) -> Result<(), Box<dyn Error>>
    {
        self.width = width;
        self.height = height;
        self.frame = new_frame.to_vec();
        if width * height * 4 != new_frame.len() as u32 {
            Err(Box::new(WrongSizeErr::new("resize_surface", (width * height) as usize, new_frame.len() as usize)))
        }
        else { Ok(()) }
    }

}


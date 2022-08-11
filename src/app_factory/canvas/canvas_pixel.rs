use crate::app_factory::canvas::Canvas;
use async_trait::async_trait;

pub struct CanvasPixel
{
    width: u32,
    height: u32,
    frame: Vec<u8>
}

#[async_trait]
impl Canvas for CanvasPixel
{
    async fn new(width: u32, height: u32, init_frame: &[u8]) -> Self where Self: Sized
    {
        CanvasPixel
        {
            width,
            height,
            frame: init_frame.to_vec()
        }
    }
    fn point(&mut self, x: i32, y: i32, color: [u8; 4])
    {
        let i: usize = x as usize * 4 + y as usize * self.width as usize * 4;

        self.frame[i..i + 4].copy_from_slice(&color);
    }

    fn copy_to_buffer(&mut self, surface: &mut [u8])
    {
        surface.copy_from_slice(&self.frame);
    }

    fn get_frame(&mut self) -> &mut [u8] { &mut self.frame[..] }

    fn resize_surface(&mut self, width: u32, height: u32, new_frame: &[u8])
    {
        self.width = width;
        self.height = height;
        self.frame = new_frame.to_vec();
    }
}


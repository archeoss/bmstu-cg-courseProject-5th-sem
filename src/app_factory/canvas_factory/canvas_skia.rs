use crate::errors;
// use async_trait::async_trait;
use crate::app_factory::canvas_factory::canvas::Canvas;
use errors::wrong_size_err::WrongSizeErr;
use std::error::Error;

pub struct CanvasSkia
{
    width: u32,
    height: u32,
    frame: Vec<u8>,
}

// #[async_trait]
impl Canvas for CanvasSkia
{
    /*async*/
    fn new(width: u32, height: u32) -> Self
    where
        Self: Sized,
    {
        CanvasSkia {
            width,
            height,
            frame: vec![0 as u8; (width * height * 4) as usize],
        }
    }
    fn point(&mut self, x: i32, y: i32, color: [u8; 4])
    {
        let i = ((x + y * self.width as i32) * 4) as usize;

        if i + 3 < self.frame.len() && i > 0 {
            self.frame[i..i + 4].copy_from_slice(&color);
        }
    }

    fn copy_to_buffer(&self, surface: &mut [u8])
    {
        surface.copy_from_slice(&self.frame);
    }

    fn get_frame(&self) -> &[u8]
    {
        &self.frame[..]
    }

    fn resize_surface(
        &mut self,
        width: u32,
        height: u32,
        new_frame: &[u8],
    ) -> Result<(), Box<dyn Error>>
    {
        if width % self.width == 0 && height % self.height == 0 {
            self.width = width;
            self.height = height;
            self.frame = new_frame.to_vec();
            if (width * height) as u32 != new_frame.len() as u32 {
                Err(Box::new(WrongSizeErr::new(
                    "resize_surface",
                    (width * height) as usize,
                    new_frame.len() as usize,
                )))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn get_width(&self) -> u32
    {
        self.width
    }
    fn get_height(&self) -> u32
    {
        self.height
    }

    fn fill(&mut self, color: [u8; 4])
    {
        for chunk in self.frame.chunks_exact_mut(4) {
            chunk.copy_from_slice(&color);
        }
    }
}

// use async_trait::async_trait;
use std::error::Error;
// #[async_trait]
pub trait Canvas
{
    /*async*/
    fn new(width: u32, height: u32) -> Self
    where
        Self: Sized;
    fn point(&mut self, x: i32, y: i32, color: [u8; 4]);
    fn copy_to_buffer(&self, surface: &mut [u8]);
    fn get_frame(&self) -> &[u8];
    fn resize_surface(
        &mut self,
        width: u32,
        height: u32,
        new_frame: &[u8],
    ) -> Result<(), Box<dyn Error>>;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn fill(&mut self, color: [u8; 4]);
}

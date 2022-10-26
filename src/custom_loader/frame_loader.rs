pub mod file_loader;

use std::error::Error;
// use std::error::Error;
use crate::models::frame_model::{Edge, Point};

pub trait FrameLoader
{
    fn open(&mut self, filename: &str) -> Result<(), Box<dyn Error>>;
    fn is_open(&self) -> bool;
    fn close(&mut self);

    fn read_points(&mut self) -> Result<Vec<Point>, Box<dyn Error>>;
    fn read_edges(&mut self) -> Result<Vec<Edge>, Box<dyn Error>>;
}

pub mod file_loader;

use std::error::Error;
// use std::error::Error;
use crate::models::frame_model::{triangle::Triangle, Edge, Point};

pub trait FrameLoader
{
    fn open(&mut self, filename: &str) -> Result<(), Box<dyn Error>>;
    fn is_open(&self) -> bool;
    fn close(&mut self);

    fn read_points(&mut self) -> Result<Vec<Point<f64>>, Box<dyn Error>>;
    fn read_edges(&mut self) -> Result<Vec<Edge>, Box<dyn Error>>;
    fn read_triang(&mut self) -> Result<Vec<Triangle>, Box<dyn Error>>;
}

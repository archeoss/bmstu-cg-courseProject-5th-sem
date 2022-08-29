pub mod file_loader;

use std::error::Error;
use crate::models::frame_model::{Edge, Point};

pub trait FrameLoader
{
    fn open(&mut self, filename: &str) -> std::io::Result<()>;
    fn is_open(&self) -> bool;
    fn close(&mut self);

    fn read_points(&mut self) -> std::io::Result<Vec<Point>>;
    fn read_edges(&mut self) -> std::io::Result<Vec<Edge>>;
}

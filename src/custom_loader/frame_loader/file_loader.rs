use std::error::Error;
use crate::models::model::Model;
use crate::custom_loader::frame_loader::FrameLoader;
use std::fs::File;
use crate::models::frame_model::{Edge, Point};

pub struct FileFrameLoader
{
    file: Option<File>,
}

impl FileFrameLoader
{
    pub fn new() -> FileFrameLoader
    {
        FileFrameLoader { file: None }
    }
}

impl FrameLoader for FileFrameLoader
{
    fn open(&mut self, filename: &str) -> std::io::Result<()>
    {
        match File::open(filename)
        {
            Ok(file) =>
            {
                self.file = Some(file);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn is_open(&self) -> bool
    {
        match self.file {
            None => { false }
            Some(_) => { true }
        }
    }

    fn close(&mut self)
    {
        self.file = None;
    }

    fn read_points(&mut self) -> std::io::Result<Vec<Point>>
    {
        let n = self.file.unwrap().read_int()?;
        let mut points = Vec::<Point>::with_capacity(n as usize);

        for _ in 0..n
        {
            let x = self.file.unwrap().read_float()?;
            let y = self.file.unwrap().read_float()?;
            let z = self.file.unwrap().read_float()?;
            points.push(Point::new(x, y, z));
        }

        Ok(points)
    }

    fn read_edges(&mut self) -> std::io::Result<Vec<Edge>>
    {

        let n = self.file.unwrap().read_int()?;
        let mut edges = Vec::<Edge>::with_capacity(n as usize);

        for _ in 0..n
        {
            let from = self.file.unwrap().read_float()?;
            let to = self.file.unwrap().read_float()?;
            edges.push(Edge::new(from, to));
        }

        Ok(edges)
    }
}
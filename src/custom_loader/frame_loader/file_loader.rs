use std::error::Error;
// use std::error::Error;
// use crate::models::model::Model;
use crate::custom_loader::frame_loader::FrameLoader;
use crate::errors::read_error::ReadErr;
use crate::models::frame_model::triangle::Triangle;
use crate::models::frame_model::{Edge, Point};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct FileFrameLoader
{
    // file: Option<File>,
    filename: String,
    buffer: Option<BufReader<File>>,
}

impl FileFrameLoader
{
    #[must_use]
    pub const fn new() -> Self
    {
        Self {
            /*file: None,*/ buffer: None,
            filename: String::new(),
        }
    }
}

impl FrameLoader for FileFrameLoader
{
    fn open(&mut self, filename: &str) -> Result<(), Box<dyn Error>>
    {
        println!("{}", env::current_dir()?.display());
        match File::open(filename) {
            Ok(file) => {
                self.filename = filename.to_string();
                self.buffer = Some(BufReader::new(file));
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn is_open(&self) -> bool
    {
        self.buffer.is_some()
    }

    fn close(&mut self)
    {
        // self.file = None;
        self.buffer = None;
    }

    fn read_points(&mut self) -> Result<Vec<Point<f64>>, Box<dyn Error>>
    {
        let Some(reader) = self.buffer.as_mut() else {
            return Err(Box::new(ReadErr::new(
                    stringify!(read_points),
                    self.filename.clone(),
                )));
        };

        let mut line = String::new();
        reader.read_line(&mut line)?;
        let n: usize = line.trim().parse()?;
        let mut points = Vec::<Point<f64>>::with_capacity(n);

        for _ in 0..n {
            line = String::new();
            reader.read_line(&mut line)?;
            let mut parts = line.split_whitespace().map(str::parse);
            match (parts.next(), parts.next(), parts.next()) {
                (Some(Ok(x)), Some(Ok(y)), Some(Ok(z))) => {
                    points.push(Point::new(x, y, z));
                }
                _ => {
                    return Err(Box::new(ReadErr::new("read_points", self.filename.clone())));
                }
            }
        }

        Ok(points)
    }

    fn read_edges(&mut self) -> Result<Vec<Edge>, Box<dyn Error>>
    {
        let Some(reader) = self.buffer.as_mut() else {
            return Err(Box::new(ReadErr::new(
                    stringify!(read_points),
                    self.filename.clone(),
                )));
        };

        let mut line = String::new();
        reader.read_line(&mut line)?;
        let n: usize = line.trim().parse()?;
        let mut edges = Vec::<Edge>::with_capacity(n);

        for _ in 0..n {
            line = String::new();
            reader.read_line(&mut line)?;
            let mut parts = line.split_whitespace().map(str::parse);

            match (parts.next(), parts.next()) {
                (Some(Ok(p1)), Some(Ok(p2))) => {
                    edges.push(Edge::new(p1, p2));
                }
                _ => {
                    return Err(Box::new(ReadErr::new("read_edges", self.filename.clone())));
                }
            }
            if let Some(Ok(_)) = parts.next() {
                return Err(Box::new(ReadErr::new("read_edges", self.filename.clone())));
            }
        }

        Ok(edges)
    }

    fn read_triang(&mut self) -> Result<Vec<Triangle>, Box<dyn Error>>
    {
        let Some(reader) = self.buffer.as_mut() else {
            return Err(Box::new(ReadErr::new(
                    stringify!(read_points),
                    self.filename.clone(),
                )));
        };

        let mut line = String::new();
        reader.read_line(&mut line)?;
        let n: usize = line.trim().parse()?;
        let mut edges = Vec::<Triangle>::with_capacity(n);

        for _ in 0..n {
            line = String::new();
            reader.read_line(&mut line)?;
            let mut parts = line.split_whitespace().map(str::parse);

            match (parts.next(), parts.next(), parts.next()) {
                (Some(Ok(p1)), Some(Ok(p2)), Some(Ok(p3))) => {
                    edges.push(Triangle::new(&[p1, p2, p3]));
                }
                _ => {
                    return Err(Box::new(ReadErr::new("read_edges", self.filename.clone())));
                }
            }
            if let Some(Ok(_)) = parts.next() {
                return Err(Box::new(ReadErr::new("read_edges", self.filename.clone())));
            }
        }

        Ok(edges)
    }
}

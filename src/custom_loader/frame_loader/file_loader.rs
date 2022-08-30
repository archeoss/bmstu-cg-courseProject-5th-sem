use std::error::Error;
// use std::error::Error;
// use crate::models::model::Model;
use crate::custom_loader::frame_loader::FrameLoader;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, os};
use crate::models::frame_model::{Edge, Point};

pub struct FileFrameLoader
{
    // file: Option<File>,
    buffer: Option<BufReader<File>>
}

impl FileFrameLoader
{
    pub fn new() -> FileFrameLoader
    {
        FileFrameLoader { /*file: None,*/ buffer: None }
    }

}

impl FrameLoader for FileFrameLoader
{
    fn open(&mut self, filename: &str) -> Result<(), Box<dyn Error>>
    {
        println!("{}", env::current_dir()?.display());
        match File::open(filename)
        {
            Ok(file) =>
            {
                self.buffer = Some(BufReader::new(file));
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn is_open(&self) -> bool
    {
        match self.buffer {
            None => { false }
            Some(_) => { true }
        }
    }

    fn close(&mut self)
    {
        // self.file = None;
        self.buffer = None;
    }

    fn read_points(&mut self) -> Result<Vec<Point>, Box<dyn Error>>
    {
        let mut reader = self.buffer.as_mut().unwrap();

        let mut line = String::new();
        reader.read_line(&mut line)?;
        let n: i64 = line.trim().parse()?;
        let mut points = Vec::<Point>::with_capacity(n as usize);

        for _ in 0..n
        {
            line = "".to_string();
            reader.read_line(&mut line)?;
            let mut parts = line.trim().split_whitespace().map(|x| x.parse::<f32>());
            match (parts.next(), parts.next(), parts.next())
            {
                (Some(Ok(x)), Some(Ok(y)), Some(Ok(z))) =>
                {
                    points.push(Point::new(x, y, z));
                }
                _ => { return Err("Invalid point".into()); }
            }
        }

        Ok(points)
    }

    fn read_edges(&mut self) -> Result<Vec<Edge>, Box<dyn Error>>
    {
        let mut reader = self.buffer.as_mut().unwrap();

        let mut line = String::new();
        reader.read_line(&mut line)?;
        let n: i64 = line.trim().parse()?;
        let mut edges = Vec::<Edge>::with_capacity(n as usize);

        for _ in 0..n
        {
            line = "".to_string();
            reader.read_line(&mut line)?;
            let mut parts = line.trim().split_whitespace().map(|x| x.parse::<usize>());

            match (parts.next(), parts.next())
            {
                (Some(Ok(p1)), Some(Ok(p2))) =>
                {
                    edges.push(Edge::new(p1, p2));
                }
                _ => { return Err("Invalid edge".into()); }
            }

            match parts.next()
            {
                Some(Ok(_)) => { return Err("Invalid edge".into()); }
                _ => {}
            }
        }

        Ok(edges)
    }
}
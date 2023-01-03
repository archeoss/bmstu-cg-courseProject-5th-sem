use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct WrongSizeErr
{
    method: String,
    frame_size: usize,
    buffer_size: usize,
}

impl WrongSizeErr
{
    #[must_use]
    pub fn new(method: &str, frame_size: usize, buffer_size: usize) -> Self
    {
        Self {
            method: method.to_string(),
            frame_size,
            buffer_size,
        }
    }
}

impl Debug for WrongSizeErr
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Wrong Size: {{ file : ile!(, line: {}, method: {}, buffer size: {}, frame size: {} }}",
            line!(),
            self.method,
            self.buffer_size,
            self.frame_size
        )
    }
}

impl Display for WrongSizeErr
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Incorrect size. Buffer size: {}, Frame size: {}",
            self.buffer_size, self.frame_size
        )
    }
}

impl Error for WrongSizeErr
{
    fn description(&self) -> &str
    {
        &self.method
    }
}

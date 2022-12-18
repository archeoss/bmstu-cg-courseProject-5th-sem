use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct ReadErr
{
    method: String,
    filename: String,
}

impl ReadErr
{
    #[must_use] pub fn new(method: &str, filename: String) -> Self
    {
        Self {
            method: method.to_string(),
            filename,
        }
    }
}

impl Debug for ReadErr
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Incorrect: {{ file : ile!(, line: {}, method: {}, filename: {} }}",
            line!(),
            self.method,
            self.filename
        )
    }
}

impl Display for ReadErr
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Can't Read, Incorrect file. Filename: {} ",
            self.filename
        )
    }
}

impl Error for ReadErr
{
    fn description(&self) -> &str
    {
        &self.method
    }
}

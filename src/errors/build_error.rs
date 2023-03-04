use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct BuildErr
{
    method: String,
    desc: String,
}

impl BuildErr
{
    #[must_use]
    pub fn new(method: &str, desc: String) -> Self
    {
        Self {
            method: method.to_string(),
            desc,
        }
    }
}

impl Debug for BuildErr
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Incorrect: {{ file : line: {}, method: {}, desc: {} }}",
            line!(),
            self.method,
            self.desc
        )
    }
}

impl Display for BuildErr
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Can't Build. Description: {} ", self.desc)
    }
}

impl Error for BuildErr
{
    fn description(&self) -> &str
    {
        &self.method
    }
}

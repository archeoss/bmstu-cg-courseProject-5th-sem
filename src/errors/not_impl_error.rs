use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct NotImplError
{
    method: String,
}

impl NotImplError
{
    #[must_use]
    pub fn new(method: &str) -> Self
    {
        Self {
            method: method.to_string(),
        }
    }
}

impl Debug for NotImplError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Not Implemented: {{ file : ile!(, line: {}, details: {} }}",
            line!(),
            self.method
        )
    }
}

impl Display for NotImplError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "The method '{}' hasn't been implemented", self.method)
    }
}

impl Error for NotImplError
{
    fn description(&self) -> &str
    {
        &self.method
    }
}

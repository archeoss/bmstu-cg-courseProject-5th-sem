use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct FocusErr
{
    method: String,
    max_focus: isize,
    get_focus: isize,
}

impl FocusErr
{
    #[must_use] pub fn new(method: &str, max_focus: isize, get_focus: isize) -> Self
    {
        Self {
            method: method.to_string(),
            max_focus,
            get_focus,
        }
    }
}

impl Debug for FocusErr
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Wrong focus: {{ file : ile!(, line: {}, method: {}, max_focus: {}, min_focus: 0, get_focus: {} }}",
            line!(),
            self.method,
            self.max_focus,
            self.get_focus
        )
    }
}

impl Display for FocusErr
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Incorrect focus. Number of models: {}, Focus: {}",
            self.max_focus, self.get_focus
        )
    }
}

impl Error for FocusErr
{
    fn description(&self) -> &str
    {
        &self.method
    }
}

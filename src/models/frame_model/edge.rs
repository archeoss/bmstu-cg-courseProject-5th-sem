use crate::macros::*;

#[derive(Copy, Clone, Debug)]
pub struct Edge
{
    from: usize,
    to: usize,
}

impl Edge
{
    #[must_use]
    pub const fn new(from: usize, to: usize) -> Self
    {
        Self { from, to }
    }

    getter_setter!(from: usize, to: usize);

    #[must_use]
    pub const fn from_to(&self) -> (usize, usize)
    {
        (self.from, self.to)
    }

    pub fn set_from_to(&mut self, from: usize, to: usize)
    {
        self.from = from;
        self.to = to;
    }
}

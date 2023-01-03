#[derive(Copy, Clone)]
pub struct Edge
{
    pub from: usize,
    pub to: usize,
}

impl Edge
{
    #[must_use]
    pub fn new(from: usize, to: usize) -> Self
    {
        Self { from, to }
    }

    #[must_use]
    pub fn get_from(&self) -> usize
    {
        self.from
    }

    #[must_use]
    pub fn get_to(&self) -> usize
    {
        self.to
    }

    pub fn set_from(&mut self, from: usize)
    {
        self.from = from;
    }

    pub fn set_to(&mut self, to: usize)
    {
        self.to = to;
    }

    #[must_use]
    pub fn get_from_to(&self) -> (usize, usize)
    {
        (self.from, self.to)
    }

    pub fn set_from_to(&mut self, from: usize, to: usize)
    {
        self.from = from;
        self.to = to;
    }
}

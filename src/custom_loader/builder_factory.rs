use std::error::Error;

pub mod model_builder;

pub trait Builder<T>
{
    fn build(&mut self) -> Result<Box<T>, dyn Error>;
}
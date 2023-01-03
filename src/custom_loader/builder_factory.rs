use std::error::Error;

pub mod model_builder;

pub trait Builder<T>
{
    fn build(&mut self, name: String) -> Result<Box<T>, Box<dyn Error>>;
}

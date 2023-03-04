use std::error::Error;

pub mod model_builder;

pub trait Builder<T, K>
{
    fn build(&mut self, name: String, types: K) -> Result<Box<T>, Box<dyn Error>>;
}

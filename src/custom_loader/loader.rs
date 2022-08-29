use std::error::Error;
use crate::models::frame_model::FrameModel;

pub mod model_loader;

pub trait Loader<T>
{
    fn load(&self, filename: &str) -> Result<Box<T>, Box<dyn Error>>;
}


pub trait LoaderFactory<T>
{
    fn create() -> Result<Box<dyn Loader<T>>, dyn Error>;
}

pub struct FrameLoaderFactory;

impl LoaderFactory<FrameModel> for FrameLoaderFactory
{
    fn create() -> Box<dyn Loader<FrameModel>>
    {
        Box::new(model_loader::ModelLoader::new())
    }
}

use crate::models::frame_model::FrameModel;
use std::error::Error;

pub mod model_loader;

pub trait Loader<T>
{
    fn load(&self, filename: &str) -> Result<Box<T>, Box<dyn Error>>;
}

pub trait LoaderFactory<T>
{
    fn create() -> Result<Box<dyn Loader<T>>, Box<dyn Error>>;
}

pub struct FrameLoaderFactory;

impl LoaderFactory<FrameModel> for FrameLoaderFactory
{
    fn create() -> Result<Box<dyn Loader<FrameModel>>, Box<dyn Error>>
    {
        Ok(Box::new(model_loader::ModelLoader::new()))
    }
}

use crate::custom_loader::loader::{FrameLoaderFactory, LoaderFactory};
use crate::models::frame_model::FrameFigure;
use crate::models::model::Model;
use std::error::Error;
pub struct LoadManager;
impl LoadManager
{
    // ...
    pub fn new() -> Self
    {
        LoadManager
    }

    pub fn load(
        &mut self,
        path: &str,
        model_type: &str,
    ) -> Result<Box<dyn Model<Output = FrameFigure>>, Box<dyn Error>>
    {
        match model_type {
            "frame" => {
                let loader = FrameLoaderFactory::create()?;
                let model = loader.load(path)?;

                Ok(model)
            }
            _ => Err("Unknown model type".into()),
        }
    }
}

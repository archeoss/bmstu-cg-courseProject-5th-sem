use crate::custom_loader::loader::{FrameLoaderFactory, LoaderFactory};
use crate::models::frame_model::FrameModel;
use crate::models::model::Model;
use crate::objects::revolution::{BodiesOfRevolution, RevolutionBuilder};
use std::error::Error;
pub struct LoadManager;

impl Default for LoadManager
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl LoadManager
{
    // ...
    #[must_use]
    pub const fn new() -> Self
    {
        Self
    }

    pub fn load(
        &mut self,
        path: &str,
        model_type: &str,
        color: [u8; 4],
    ) -> Result<Box<dyn Model<Output = FrameModel>>, Box<dyn Error>>
    {
        match model_type {
            "frame" => {
                let loader = FrameLoaderFactory::create()?;
                let model = loader.load(path)?;

                Ok(RevolutionBuilder::new().build(
                    "AbstractModel".to_string(),
                    BodiesOfRevolution::AbstractModel(*model),
                    color,
                )?)
            }
            _ => Err("Unknown model type".into()),
        }
    }
}

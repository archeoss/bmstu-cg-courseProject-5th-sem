use std::error::Error;
use std::fs::File;
use crate::custom_loader::loader::{FrameLoaderFactory, LoaderFactory};
use crate::models::model::Model;
struct LoadManager;
impl LoadManager {
    // ...
    pub fn load(&mut self, path: &str, model_type: &str) -> Result<Box<dyn Model>, dyn Error>
    {
        match model_type
        {
            "frame" => {
                let mut loader = FrameLoaderFactory::create();
                let model = loader.load(model_type)?;

                Ok(model)
            },
            _ => Err("Unknown model type".into())
        }
    }
}
use std::error::Error;
use crate::custom_loader::builder_factory::Builder;
use crate::custom_loader::frame_loader::file_loader::FileFrameLoader;
use crate::custom_loader::frame_loader::FrameLoader;
use crate::custom_loader::loader::Loader;
use crate::models::frame_model::FrameModel;
use crate::custom_loader::builder_factory::model_builder::FrameModelBuilder;

pub struct ModelLoader;

impl ModelLoader
{
    pub fn new() -> Self
    {
        ModelLoader
    }
}

impl Loader<FrameModel> for ModelLoader
{
    fn load(&self, filename: &str) -> Result<Box<FrameModel>, Box<dyn Error>>
    {
        let mut file_loader = FileFrameLoader::new();
        file_loader.open(filename)?;
        let points = file_loader.read_points()?;
        let edges = file_loader.read_edges()?;
        file_loader.close();

        let mut builder = FrameModelBuilder::new();

        builder.add_points(&points).add_edges(&edges).build()
    }
}
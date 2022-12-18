use crate::custom_loader::builder_factory::Builder;
use crate::models::frame_model::{Edge, FrameFigure, FrameModel, Point};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub struct FrameModelBuilder
{
    points: Option<Vec<Point>>,
    edges: Option<Vec<Edge>>,
    model: Option<FrameModel>,
}

impl FrameModelBuilder
{
    #[must_use] pub fn new() -> Self
    {
        Self {
            points: None,
            edges: None,
            model: None,
        }
    }

    pub fn add_points(&mut self, points: &Vec<Point>) -> &mut Self
    {
        self.points = Some(points.clone());

        self
    }

    pub fn add_edges(&mut self, edges: &Vec<Edge>) -> &mut Self
    {
        self.edges = Some(edges.clone());

        self
    }
}

impl Builder<FrameModel> for FrameModelBuilder
{
    fn build(&mut self) -> Result<Box<FrameModel>, Box<dyn Error>>
    {
        let figure = FrameFigure::new_with_points_and_edges(
            self.points.clone().ok_or("")?,
            self.edges.clone().ok_or("")?,
        );
        let figure = Rc::new(RefCell::new(figure));

        Ok(Box::new(FrameModel::new(figure)))
    }
}

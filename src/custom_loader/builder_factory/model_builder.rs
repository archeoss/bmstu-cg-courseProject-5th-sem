use crate::custom_loader::builder_factory::Builder;
use crate::models::frame_model::{Edge, FrameFigure, FrameModel, Point};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[allow(clippy::module_name_repetitions)]
pub struct FrameModelBuilder
{
    points: Option<Vec<Point<f64>>>,
    edges: Option<Vec<Edge>>,
    model: Option<FrameModel>,
}

impl FrameModelBuilder
{
    #[must_use]
    pub const fn new() -> Self
    {
        Self {
            points: None,
            edges: None,
            model: None,
        }
    }

    pub fn add_points(&mut self, points: &[Point<f64>]) -> &mut Self
    {
        self.points = Some(points.to_vec());
        self.model = None;

        self
    }

    pub fn add_edges(&mut self, edges: &[Edge]) -> &mut Self
    {
        self.edges = Some(edges.to_vec());
        self.model = None;

        self
    }
}

impl Builder<FrameModel> for FrameModelBuilder
{
    fn build(&mut self, name: String) -> Result<Box<FrameModel>, Box<dyn Error>>
    {
        if let Some(model) = self.model.as_ref() {
            Ok(Box::new(model.clone()))
        } else {
            let figure = FrameFigure::new_with_points_and_edges(
                self.points.clone().ok_or("")?,
                self.edges.clone().ok_or("")?,
            );
            let figure = Rc::new(RefCell::new(figure));

            Ok(Box::new(FrameModel::new(figure, name)))
        }
    }
}

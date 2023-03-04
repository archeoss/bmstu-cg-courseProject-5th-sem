use crate::custom_loader::builder_factory::Builder;
use crate::errors;
use crate::models::frame_model::triangle::Triangle;
use crate::models::frame_model::{Edge, FrameFigure, FrameModel, Point};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[derive(Debug)]
pub enum Models
{
    FrameModel,
}

#[allow(clippy::module_name_repetitions)]
pub struct FrameModelBuilder
{
    points: Option<Vec<Point<f64>>>,
    edges: Option<Vec<Edge>>,
    triangles: Option<Vec<Triangle>>,
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
            triangles: None,
            model: None,
        }
    }

    pub fn add_points(&mut self, points: &[Point<f64>]) -> &mut Self
    {
        self.points = Some(points.to_vec());
        self.model = None;

        self
    }

    pub fn add_triangles(&mut self, triangles: &[Triangle]) -> &mut Self
    {
        self.triangles = Some(triangles.to_vec());
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

impl Builder<FrameModel, Models> for FrameModelBuilder
{
    fn build(&mut self, name: String, types: Models) -> Result<Box<FrameModel>, Box<dyn Error>>
    {
        match types {
            Models::FrameModel => {
                if let Some(model) = self.model.as_ref() {
                    Ok(Box::new(model.clone()))
                } else {
                    let mut figure = FrameFigure::new_with_points_and_edges(
                        self.points.clone().ok_or("No points")?,
                        self.edges.clone().ok_or("No edges")?,
                    );
                    figure.set_triangles(self.triangles.clone().ok_or("No triangles")?);
                    let figure = Rc::new(RefCell::new(figure));

                    Ok(Box::new(FrameModel::new(figure, name)))
                }
            }
        }
    }
}

pub mod edge;
pub mod point;

pub use crate::managers::visitor::Visitor;
pub use crate::models::model::Model;
pub use crate::objects::object::Object;
pub use crate::objects::visibility::Visibility;
use cgmath::Matrix4;
pub use edge::Edge;
pub use point::Point;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct FrameFigure
{
    points: Vec<Point>,
    edges: Vec<Edge>,
}

#[derive(Clone)]
pub struct FrameModel
{
    figure: Rc<RefCell<FrameFigure>>,
    transform: Matrix4<f32>,
}

impl FrameFigure
{
    pub fn new() -> FrameFigure
    {
        FrameFigure {
            points: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn new_with_points(points: Vec<Point>) -> FrameFigure
    {
        FrameFigure {
            points,
            edges: Vec::new(),
        }
    }

    pub fn new_with_edges(edges: Vec<Edge>) -> FrameFigure
    {
        FrameFigure {
            points: Vec::new(),
            edges,
        }
    }

    pub fn new_with_points_and_edges(points: Vec<Point>, edges: Vec<Edge>) -> FrameFigure
    {
        FrameFigure { points, edges }
    }

    pub fn get_points(&self) -> &Vec<Point>
    {
        &self.points
    }

    pub fn get_edges(&self) -> &Vec<Edge>
    {
        &self.edges
    }

    pub fn get_points_mut(&mut self) -> &mut Vec<Point>
    {
        &mut self.points
    }

    pub fn get_edges_mut(&mut self) -> &mut Vec<Edge>
    {
        &mut self.edges
    }

    pub fn add_point(&mut self, point: Point)
    {
        self.points.push(point);
    }

    pub fn add_edge(&mut self, edge: Edge)
    {
        self.edges.push(edge);
    }

    pub fn remove_point(&mut self, index: usize)
    {
        self.points.remove(index);
    }

    pub fn remove_edge(&mut self, index: usize)
    {
        self.edges.remove(index);
    }

    pub fn get_point(&self, index: usize) -> &Point
    {
        &self.points[index]
    }

    pub fn get_edge(&self, index: usize) -> &Edge
    {
        &self.edges[index]
    }

    pub fn get_point_mut(&mut self, index: usize) -> &mut Point
    {
        &mut self.points[index]
    }

    pub fn get_edge_mut(&mut self, index: usize) -> &mut Edge
    {
        &mut self.edges[index]
    }

    pub fn get_center(&self) -> Point
    {
        let mut max = self.points[0];
        let mut min = self.points[0];

        for point in &self.points {
            max = Point::new(
                max.get_x().max(point.get_x()),
                max.get_y().max(point.get_y()),
                max.get_z().max(point.get_z()),
            );
            min = Point::new(
                min.get_x().min(point.get_x()),
                min.get_y().min(point.get_y()),
                min.get_z().min(point.get_z()),
            );
        }

        (max + min) / Point::new(2.0, 2.0, 2.0)
    }
}

impl FrameModel
{
    pub(crate) fn new(figure: Rc<RefCell<FrameFigure>>) -> FrameModel
    {
        FrameModel {
            figure,
            transform: Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
        }
    }
}

impl Model for FrameModel
{
    type Output = FrameFigure;
    fn get_model(&self) -> Rc<RefCell<Self::Output>>
    {
        self.figure.clone()
    }

    fn get_center(&self) -> Point
    {
        self.figure.borrow().get_center()
    }

    fn get_transform(&self) -> Matrix4<f32>
    {
        self.transform
    }
    fn transform(&mut self, transform: Matrix4<f32>)
    {
        self.transform = self.transform * transform;
    }
}

impl Visibility for FrameModel
{
    fn is_visible(&self) -> bool
    {
        true
    }
}

impl Object for FrameModel
{
    fn add(&mut self, obj: Box<dyn Object>) -> bool
    {
        false
    }
    fn remove(&mut self, obj: Box<dyn Object>) -> bool
    {
        false
    }
    fn accept(&mut self, visitor: &mut dyn Visitor)
    {
        visitor.visit_model(self);
    }
    fn transform(&mut self, transform: Matrix4<f32>)
    {
        self.transform = self.transform * transform;
    }
    // fn get_type(&self) -> ObjectType
    // {
    //     ObjectType::FrameModel
    // }
    // fn inspect(&self) -> String
    // {
    //     String::from("FrameModel")
    // }
}

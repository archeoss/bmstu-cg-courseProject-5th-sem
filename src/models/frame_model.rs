pub mod edge;
pub mod point;
pub mod triangle;
pub use crate::managers::visitor::Visitor;
pub use crate::models::model::Model;
pub use crate::objects::object::Object;
pub use crate::objects::visibility::Visibility;
use nalgebra::Matrix4;
// use cgmath::Matrix4;
use self::triangle::Triangle;
use crate::macros::{getter, getter_ref, getter_setter, setter};
pub use edge::Edge;
pub use point::Point;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug)]
pub enum State {
    Union,
    Subtract,
    Intersect,
}

#[derive(Clone, Debug)]
pub struct FrameFigure {
    points: Vec<Point<f64>>,
    edges: Vec<Edge>,
    cached_points: Vec<Point<f64>>,
    triangles: Vec<Triangle>,
    normals: Vec<Point<f64>>,
    state: State,
    color: [u8; 4],
}

#[derive(Clone, Debug)]
pub struct FrameModel {
    name: String,
    figures: Vec<Rc<RefCell<FrameFigure>>>,
    transform: Matrix4<f64>,
    is_cached: bool,
}

impl Default for FrameFigure {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameFigure {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            points: Vec::new(),
            edges: Vec::new(),
            cached_points: vec![],
            triangles: vec![],
            normals: vec![],
            state: State::Union,
            color: [255; 4],
        }
    }

    #[must_use]
    pub fn new_with_points(points: Vec<Point<f64>>) -> Self {
        Self {
            points,
            edges: Vec::new(),
            cached_points: vec![],
            triangles: vec![],
            normals: vec![],
            state: State::Union,
            color: [255; 4],
        }
    }

    #[must_use]
    pub fn new_with_edges(edges: Vec<Edge>) -> Self {
        Self {
            points: Vec::new(),
            edges,
            cached_points: vec![],
            triangles: vec![],
            normals: vec![],
            state: State::Union,
            color: [255; 4],
        }
    }

    #[must_use]
    pub fn new_with_points_and_edges(points: Vec<Point<f64>>, edges: Vec<Edge>) -> Self {
        Self {
            points,
            edges,
            cached_points: vec![],
            triangles: vec![],
            normals: vec![],
            state: State::Union,
            color: [255; 4],
        }
    }

    getter!(color: [u8; 4]);

    setter!(
        points: Vec<Point<f64>>,
        edges: Vec<Edge>,
        cached_points: Vec<Point<f64>>,
        triangles: Vec<Triangle>,
        normals: Vec<Point<f64>>,
        color: [u8; 4]
    );

    getter_ref!(
        points: Vec<Point<f64>>,
        edges: Vec<Edge>,
        cached_points: Vec<Point<f64>>,
        triangles: Vec<Triangle>,
        normals: Vec<Point<f64>>
    );

    #[must_use]
    pub const fn name(&self) -> &str {
        "FrameFigure"
    }

    #[must_use]
    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn triangles_mut(&mut self) -> &mut Vec<Triangle> {
        &mut self.triangles
    }

    pub fn points_mut(&mut self) -> &mut Vec<Point<f64>> {
        &mut self.points
    }

    pub fn edges_mut(&mut self) -> &mut Vec<Edge> {
        &mut self.edges
    }

    pub fn add_point(&mut self, point: Point<f64>) {
        self.points.push(point);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn remove_point(&mut self, index: usize) {
        self.points.remove(index);
    }

    pub fn remove_edge(&mut self, index: usize) {
        self.edges.remove(index);
    }

    #[must_use]
    pub fn point(&self, index: usize) -> &Point<f64> {
        &self.points[index]
    }

    #[must_use]
    pub fn edge(&self, index: usize) -> &Edge {
        &self.edges[index]
    }

    pub fn point_mut(&mut self, index: usize) -> &mut Point<f64> {
        &mut self.points[index]
    }

    pub fn edge_mut(&mut self, index: usize) -> &mut Edge {
        &mut self.edges[index]
    }

    pub fn compute_normals(&mut self) -> &Vec<Point<f64>> {
        for triangle in &self.triangles {
            let a = self.points[triangle.a()];
            let b = self.points[triangle.b()];
            let c = self.points[triangle.c()];

            let ab = b - a;
            let ac = c - a;
            let mut normal = Point::new(
                ab.y().mul_add(ac.z(), -ab.z() * ac.y()),
                ab.z().mul_add(ac.x(), -ab.x() * ac.z()),
                ab.x().mul_add(ac.y(), -ab.y() * ac.x()),
            );
            normal.normalize();
            self.normals.push(normal);
        }

        &self.normals
    }

    #[must_use]
    pub fn center(&self) -> Point<f64> {
        let mut max = self.points[0];
        let mut min = self.points[0];

        for point in &self.points {
            max = Point::new(
                max.x().max(point.x()),
                max.y().max(point.y()),
                max.z().max(point.z()),
            );
            min = Point::new(
                min.x().min(point.x()),
                min.y().min(point.y()),
                min.z().min(point.z()),
            );
        }

        (max + min) / Point::new(2.0, 2.0, 2.0)
    }

    fn update_cached(&mut self, transform: &Matrix4<f64>) {
        self.cached_points = self
            .points
            .clone()
            .into_iter()
            .map(|pnt| pnt.transform(transform))
            .collect();
        self.compute_normals();
    }
}

impl FrameModel {
    pub(crate) fn new(figure: Rc<RefCell<FrameFigure>>, name: String) -> Self {
        Self {
            figures: vec![figure],
            transform: Matrix4::identity(),
            name,
            is_cached: false,
        }
    }
}

impl Model for FrameModel {
    type Output = FrameFigure;
    fn figures(&self) -> Vec<Rc<RefCell<Self::Output>>> {
        self.figures.clone()
    }

    fn add_figure(&mut self, model: Rc<RefCell<Self::Output>>) {
        let new_points = model.borrow().cached_points.clone();
        model.borrow_mut().points = new_points;
        for figure in &self.figures {
            let new_points = figure.borrow().cached_points.clone();
            figure.borrow_mut().points = new_points;
        }
        self.transform = Matrix4::identity();
        self.figures.push(model);
    }

    fn center(&self) -> Point<f64> {
        self.true_center().transform(&self.transform)
    }

    fn true_center(&self) -> Point<f64> {
        let mut center = Point::default();
        for figure in &self.figures {
            center += figure.borrow().center();
        }

        center
            / Point::new(
                self.figures.len() as f64,
                self.figures.len() as f64,
                self.figures.len() as f64,
            )
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn transform(&self) -> Matrix4<f64> {
        self.transform
    }

    fn transform_self(&mut self, transform: Matrix4<f64>) {
        self.is_cached = false;
        self.transform = self.transform * transform;
    }
    fn transform_first(&mut self, transform: Matrix4<f64>) {
        self.is_cached = false;
        self.transform = transform * self.transform;
    }

    fn update_model(&mut self) {
        if !self.is_cached {
            self.is_cached = true;
            for figure in &self.figures {
                figure.as_ref().borrow_mut().update_cached(&self.transform);
            }
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Visibility for FrameModel {
    fn is_visible(&self) -> bool {
        true
    }
}

impl Object for FrameModel {
    fn add(&mut self, _obj: Box<dyn Object>) -> bool {
        false
    }
    fn remove(&mut self, _obj: Box<dyn Object>) -> bool {
        false
    }
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_model(self);
    }
    fn transform_self(&mut self, transform: Matrix4<f64>) {
        self.is_cached = false;
        self.transform = self.transform * transform;
    }
    fn transform_first(&mut self, transform: Matrix4<f64>) {
        self.is_cached = false;
        self.transform = transform * self.transform;
    }
    // fn type(&self) -> ObjectType
    // {
    //     ObjectType::FrameModel
    // }
    // fn inspect(&self) -> String
    // {
    //     String::from("FrameModel")
    // }
}

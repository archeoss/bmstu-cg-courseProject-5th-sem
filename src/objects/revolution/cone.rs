use crate::{
    custom_loader::builder_factory::model_builder::FrameModelBuilder,
    macros::{getter, getter_setter, setter},
    models::{
        frame_model::{triangle, Edge, FrameFigure, FrameModel, Point},
        model::Model,
    },
};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};
const H_ANGLE: f64 = PI / 180.0 * 72.0;
// const V_ANGLE: f64 = (0.5f64).atan();

#[derive(Debug, Clone)]
pub struct Cone {
    frame_model: Rc<RefCell<FrameModel>>,
    height: f64,
    radius: f64,
    subdivision: usize,
}

impl Cone {
    pub fn new(frame_model: FrameModel, height: f64, radius: f64, subdivision: usize) -> Self {
        Self {
            frame_model: Rc::new(RefCell::new(frame_model)),
            height,
            radius,
            subdivision,
        }
    }

    getter_setter!(height: f64, radius: f64, subdivision: usize);

    pub fn subdiv(&mut self, subdivision: usize, radius: f64) {
        // iterate all subdivision levels
        let refer = self.frame_model.borrow().figures();
        let mut model = refer[0].borrow_mut();
        for i in 0..subdivision {
            // copy prev vertex/index arrays and clear
            let tmpVertices = model.points();
            let tmpIndices = model.triangles();
            let mut index = 0;
            let mut newVert = Vec::with_capacity(tmpVertices.len() * 4);
            let mut newIndeces = Vec::with_capacity(tmpIndices.len() * 4);
            let mut newEdges = Vec::with_capacity(tmpVertices.len() * 3);
            // perform subdivision for each triangle
            for j in tmpIndices {
                // get 3 vertices of a triangle
                let v1 = &tmpVertices[j.a()];
                let v2 = &tmpVertices[j.b()];
                let v3 = &tmpVertices[j.c()];

                // compute new vertice by spliting half on edge
                //         v3
                //        /|\
                //       / | \
                //      /  |  \
                //    v1---*---v2
                //       newV
                let new_v = Self::computeHalfVertex([v1, v2], radius);

                // add 2 new triangles to vertex array
                newVert.extend([*v1, new_v, *v3]);
                newVert.extend([new_v, *v2, *v3]);
                // add indices of 2 new triangles
                newIndeces.push(triangle::Triangle::new(&[index, index + 1, index + 2]));
                newIndeces.push(triangle::Triangle::new(&[index + 3, index + 4, index + 5]));
                newEdges.extend([
                    Edge::new(index + 1, index + 2),
                    Edge::new(index, index + 1),
                    Edge::new(index, index + 2),
                ]);
                newEdges.extend([
                    Edge::new(index + 3, index + 4),
                    Edge::new(index + 3, index + 5),
                    Edge::new(index + 4, index + 5),
                ]);

                index += 6; // next index
            }
            // drop(tmpVertices);
            // drop(tmpIndices);

            model.set_edges(newEdges);
            model.set_points(newVert);
            model.set_triangles(newIndeces);
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
    // find middle point of 2 vertices
    // NOTE: new vertex must be resized, so the length is equal to the radius
    ///////////////////////////////////////////////////////////////////////////////
    fn computeHalfVertex(vs: [&Point<f64>; 2], radius: f64) -> Point<f64> {
        let mut new_v = *vs[0] + *vs[1]; // x
        let scale = radius / (new_v.x().powi(2) + new_v.y().powi(2) + new_v.z().powi(2)).sqrt();
        new_v *= Point::new(scale, scale, scale);

        new_v
    }
}

impl Model for Cone {
    type Output = FrameModel;

    fn figures(&self) -> Vec<Rc<RefCell<Self::Output>>> {
        vec![self.frame_model.clone()]
    }

    fn true_center(&self) -> Point<f64> {
        self.frame_model.borrow().true_center()
    }

    fn center(&self) -> Point<f64> {
        self.frame_model.borrow().center()
    }

    fn name(&self) -> String {
        self.frame_model.borrow().name()
    }

    fn transform(&self) -> nalgebra::Matrix4<f64> {
        self.frame_model.borrow().transform()
    }

    fn transform_self(&mut self, transform: nalgebra::Matrix4<f64>) {
        self.frame_model.borrow_mut().transform_self(transform);
    }

    fn transform_first(&mut self, transform: nalgebra::Matrix4<f64>) {
        self.frame_model.borrow_mut().transform_first(transform);
    }

    fn update_model(&mut self) {
        self.frame_model.borrow_mut().update_model();
    }

    fn add_figure(&mut self, model: Rc<RefCell<Self::Output>>) {
        for figure in model.borrow().figures() {
            self.frame_model.borrow_mut().add_figure(figure);
        }
    }

    fn set_name(&mut self, name: &str) {
        self.frame_model.borrow_mut().set_name(name);
    }
}
//
impl From<Rc<RefCell<FrameModel>>> for Cone {
    fn from(frame_model: Rc<RefCell<FrameModel>>) -> Self {
        let (mut minr, mut maxr) = (Point::default(), Point::default());
        let (mut minh, mut maxh) = (Point::default(), Point::default());
        {
            let model = frame_model.borrow().figures()[0].clone();
            let figure = model.borrow();
            let points = figure.points();
            (maxr, minr) = (
                points
                    .iter()
                    .filter(|p| p.y() < 0.1)
                    .max_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
                    .unwrap()
                    .clone(),
                points
                    .iter()
                    .filter(|p| p.y() < 0.1)
                    .min_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
                    .unwrap()
                    .clone(),
            );
            (maxh, minh) = (
                points
                    .iter()
                    .max_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
                    .unwrap()
                    .clone(),
                points
                    .iter()
                    .min_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
                    .unwrap()
                    .clone(),
            );
        }
        Self {
            frame_model,
            radius: maxr.x() - minr.x(),
            height: maxh.y() - minh.y(),
            subdivision: 0,
        }
    }
}

impl From<Cone> for Rc<RefCell<FrameModel>> {
    fn from(sphere: Cone) -> Self {
        sphere.figures()[0].clone()
    }
}

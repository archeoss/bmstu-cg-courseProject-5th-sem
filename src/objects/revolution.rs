use crate::{
    custom_loader::builder_factory::{
        model_builder::{FrameModelBuilder, Models},
        Builder,
    },
    errors::{self, build_error::BuildErr},
    models::{
        frame_model::{triangle::Triangle, Edge, FrameFigure, FrameModel, Point},
        model::Model,
    },
};
pub mod abstract_obj;
pub mod cone;
pub mod cylinder;
pub mod sphere;
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use self::{abstract_obj::AbstractModel, cone::Cone, cylinder::Cylinder, sphere::Sphere};
const H_ANGLE: f64 = PI / 180.0 * 72.0;
#[derive(Debug)]
pub enum BodiesOfRevolution {
    Sphere(f64, usize),
    Cone(f64, f64, usize),
    Cylinder(f64, f64, f64, usize),
    AbstractModel(FrameModel),
}

pub struct RevolutionBuilder {
    points: Option<Vec<Point<f64>>>,
    edges: Option<Vec<Edge>>,
    triangles: Option<Vec<Triangle>>,
    model: Option<Rc<RefCell<FrameModel>>>,
}

impl RevolutionBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            points: None,
            edges: None,
            model: None,
            triangles: None,
        }
    }

    pub fn add_triangles(&mut self, triangles: &[Triangle]) -> &mut Self {
        self.triangles = Some(triangles.to_vec());
        self.model = None;

        self
    }

    pub fn add_points(&mut self, points: &[Point<f64>]) -> &mut Self {
        self.points = Some(points.to_vec());
        self.model = None;

        self
    }

    pub fn add_edges(&mut self, edges: &[Edge]) -> &mut Self {
        self.edges = Some(edges.to_vec());
        self.model = None;

        self
    }

    pub fn triangles(&self) -> Result<&[Triangle], BuildErr> {
        if let Some(triangles) = &self.triangles {
            Ok(triangles.as_slice())
        } else {
            Err(BuildErr::new(
                "triangles",
                "There is no triangles in Builder".to_string(),
            ))
        }
    }

    pub fn edges(&self) -> Result<&[Edge], BuildErr> {
        if let Some(edges) = &self.edges {
            Ok(edges.as_slice())
        } else {
            Err(BuildErr::new(
                "edges",
                "There is no edges in Builder".to_string(),
            ))
        }
    }

    pub fn points(&self) -> Result<&[Point<f64>], BuildErr> {
        if let Some(points) = &self.points {
            Ok(points.as_slice())
        } else {
            Err(BuildErr::new(
                "points",
                "There is no points in Builder".to_string(),
            ))
        }
    }
    pub fn generate_cone(&mut self, radius: f64, height: f64) -> &mut Self {
        let mut points = vec![Point::default(); 12];
        let mut edges: Vec<Edge> = Vec::new();
        let mut h_angle1 = -PI / 2.0 - H_ANGLE / 2.0; // start from -126 deg at 1st row
        let mut h_angle2 = -PI / 2.0; // start from -90 deg at 2nd row
                                      // let V_ANGLE: f64 = (0.5f64).atan();
        let V_ANGLE: f64 = PI / 4.0;
        points[0] = Point::new(0.0, height, 0.0);
        // compute 10 points at 1st and 2nd rows
        for i in 1..11 {
            let angle = i as f64 * 2.0 * PI / 10.0;
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            // base.push([x, 0.0, z]);
            points[i] = Point::new(x, 0.0, z); // z

            // next horizontal angles

            edges.push(Edge::new(0, i));
            edges.push(Edge::new(i, i % 10 + 1));
            edges.push(Edge::new(11, i));
            // println!("{edges:?}");
        }
        self.triangles = Some(vec![]);
        for i in 1..11 {
            let v2 = if i == 10 { 1 } else { i + 1 as usize };
            self.triangles
                .as_mut()
                .unwrap()
                .extend([Triangle::new(&[v2, i, 0]), Triangle::new(&[i, v2, 11])]);
        }
        // the last bottom vertex at (0, 0, -r)
        points[11] = Point::new(0.0, 0.0, 0.0);
        self.points = Some(points);
        self.edges = Some(edges);

        self
    }

    fn build_cone(
        &mut self,
        name: String,
        radius: f64,
        height: f64,
        subdivision: usize,
        color: [u8; 4],
    ) -> Result<Box<Cone>, Box<dyn std::error::Error>> {
        if let Some(model) = &self.model {
            return Ok(Box::from(Cone::from(model.clone())));
        }
        self.generate_cone(radius, height);
        let mut builder = FrameModelBuilder::new();
        let mut cone = Cone::new(
            *builder
                .add_points(&self.points()?)
                .add_edges(&self.edges()?)
                .add_triangles(&self.triangles()?)
                .build(name, Models::FrameModel)
                .unwrap(),
            height,
            radius,
            subdivision,
        );

        cone.figures()[0].borrow_mut().figures()[0]
            .borrow_mut()
            .set_color(color);
        cone.subdiv(subdivision, radius);
        Ok(Box::from(cone))
    }

    fn build_cylin(
        &mut self,
        name: String,
        lower_radius: f64,
        upper_radius: f64,
        height: f64,
        subdivision: usize,
        color: [u8; 4],
    ) -> Result<Box<Cylinder>, Box<dyn std::error::Error>> {
        if let Some(model) = &self.model {
            return Ok(Box::from(Cylinder::from(model.clone())));
        }
        self.generate_cylin(upper_radius, lower_radius, height, subdivision);
        let mut builder = FrameModelBuilder::new();
        let mut cone = Cylinder::new(
            *builder
                .add_points(&self.points()?)
                .add_edges(&self.edges()?)
                .add_triangles(&self.triangles()?)
                .build(name, Models::FrameModel)
                .unwrap(),
            height,
            upper_radius,
            lower_radius,
            subdivision,
        );

        cone.figures()[0].borrow_mut().figures()[0]
            .borrow_mut()
            .set_color(color);
        // cone.subdiv(subdivision, lower_radius, upper_radius);
        Ok(Box::from(cone))
    }

    pub fn generate_cylin(
        &mut self,
        lower_radius: f64,
        upper_radius: f64,
        height: f64,
        subdivision: usize,
    ) -> &mut Self {
        let n = 5 * 2usize.pow(subdivision as u32);
        let mut points = vec![Point::default(); n * 2 + 2];
        let mut edges: Vec<Edge> = Vec::new();
        let mut h_angle1 = -PI / 2.0 - H_ANGLE / 2.0; // start from -126 deg at 1st row
        let mut h_angle2 = -PI / 2.0; // start from -90 deg at 2nd row
                                      // let V_ANGLE: f64 = (0.5f64).atan();
        let V_ANGLE: f64 = PI / 4.0;
        points[0] = Point::new(0.0, height, 0.0);
        // compute 10 points at 1st and 2nd rows
        for i in 1..=n {
            let i1 = i;
            let i2 = i + n;
            let angle = i as f64 * 2.0 * PI / n as f64;
            let x_upper = upper_radius * angle.cos();
            let z_upper = upper_radius * angle.sin();
            let x_lower = lower_radius * angle.cos();
            let z_lower = lower_radius * angle.sin();
            // base.push([x, 0.0, z]);
            points[i] = Point::new(x_upper, height, z_upper); // z
            points[i2] = Point::new(x_lower, 0.0, z_lower); // z

            // next horizontal angles

            edges.push(Edge::new(0, i1));
            edges.push(Edge::new(i1, i1 % n + 1));
            // edges.push(Edge::new(i1, i1 % 5 + 6));
            edges.push(Edge::new(i2, i1 % n + n + 1));
            edges.push(Edge::new(i2, i1));
            edges.push(Edge::new(i2, i1 % n + 1));
            edges.push(Edge::new(n * 2 + 1, i2));
            // println!("{edges:?}");
        }
        self.triangles = Some(vec![]);
        for i in 1..=n {
            let i1 = i;
            let i2 = i + n;
            let v12 = i1 % n + 1;
            let v22 = i1 % n + n + 1;
            self.triangles.as_mut().unwrap().extend([
                Triangle::new(&[v12, i1, 0]),
                Triangle::new(&[i2, v22, n * 2 + 1]),
                Triangle::new(&[v12, i2, i1]),
                Triangle::new(&[v22, i2, v12]),
            ]);
        }
        // the last bottom vertex at (0, 0, -r)
        points[n * 2 + 1] = Point::new(0.0, 0.0, 0.0);

        self.points = Some(points);
        self.edges = Some(edges);

        self
    }

    pub fn generate_base(&mut self, radius: f64) -> &mut Self {
        let mut points = vec![Point::default(); 12];
        let mut edges: Vec<Edge> = Vec::new();
        let mut h_angle1 = -PI / 2.0 - H_ANGLE / 2.0; // start from -126 deg at 1st row
        let mut h_angle2 = -PI / 2.0; // start from -90 deg at 2nd row
        let V_ANGLE: f64 = (0.5f64).atan();
        points[0] = Point::new(0.0, 0.0, radius);
        // compute 10 points at 1st and 2nd rows
        for i in 1..6 {
            let i1 = i; // index for 1st row
            let i2 = i + 5; // index for 2nd row

            let z = radius * V_ANGLE.sin(); // elevaton
            let xy = radius * V_ANGLE.cos(); // length on XY plane

            points[i1] = Point::new(xy * h_angle1.cos(), xy * h_angle1.sin(), z); // z
            points[i2] = Point::new(xy * h_angle2.cos(), xy * h_angle2.sin(), -z); // z

            // next horizontal angles
            h_angle1 += H_ANGLE;
            h_angle2 += H_ANGLE;

            edges.push(Edge::new(0, i1));
            edges.push(Edge::new(i1, i1 % 5 + 1));
            // edges.push(Edge::new(i1, i1 % 5 + 6));
            edges.push(Edge::new(i2, i1 % 5 + 6));
            edges.push(Edge::new(i2, i1));
            edges.push(Edge::new(i2, i1 % 5 + 1));
            edges.push(Edge::new(11, i2));
            // println!("{edges:?}");
        }

        // the last bottom vertex at (0, 0, -r)
        points[11] = Point::new(0.0, 0.0, -radius);
        self.triangles = Some(vec![
            Triangle::new(&[0, 1, 2]),
            Triangle::new(&[0, 2, 3]),
            Triangle::new(&[0, 3, 4]),
            Triangle::new(&[0, 4, 5]),
            Triangle::new(&[0, 5, 1]),
            Triangle::new(&[1, 6, 2]),
            Triangle::new(&[2, 6, 7]),
            Triangle::new(&[2, 7, 3]),
            Triangle::new(&[3, 7, 8]),
            Triangle::new(&[3, 8, 4]),
            Triangle::new(&[4, 8, 9]),
            Triangle::new(&[4, 9, 5]),
            Triangle::new(&[5, 9, 10]),
            Triangle::new(&[5, 10, 1]),
            Triangle::new(&[1, 10, 6]),
            Triangle::new(&[6, 11, 7]),
            Triangle::new(&[7, 11, 8]),
            Triangle::new(&[8, 11, 9]),
            Triangle::new(&[9, 11, 10]),
            Triangle::new(&[10, 11, 6]),
        ]);
        self.points = Some(points);
        self.edges = Some(edges);

        self
    }

    fn build_sphere(
        &mut self,
        name: String,
        radius: f64,
        subdivision: usize,
        color: [u8; 4],
    ) -> Result<Box<Sphere>, Box<dyn std::error::Error>> {
        if let Some(model) = &self.model {
            return Ok(Box::from(Sphere::from(model.clone())));
        }
        self.generate_base(radius);
        let mut builder = FrameModelBuilder::new();
        let mut sphere = Sphere::new(
            *builder
                .add_points(&self.points()?)
                .add_edges(&self.edges()?)
                .add_triangles(&self.triangles()?)
                .build(name, Models::FrameModel)
                .unwrap(),
            radius,
            subdivision,
        );
        sphere.figures()[0].borrow_mut().figures()[0]
            .borrow_mut()
            .set_color(color);
        sphere.subdiv(subdivision, radius);
        Ok(Box::from(sphere))
    }
}

impl RevolutionBuilder {
    pub fn build(
        &mut self,
        name: String,
        types: BodiesOfRevolution,
        color: [u8; 4],
    ) -> Result<Box<dyn Model<Output = FrameModel>>, Box<dyn std::error::Error>> {
        match types {
            BodiesOfRevolution::Sphere(rad, div) => Ok(self.build_sphere(name, rad, div, color)?),
            BodiesOfRevolution::Cone(rad, height, div) => {
                Ok(self.build_cone(name, rad, height, div, color)?)
            }
            BodiesOfRevolution::Cylinder(lower_rad, upper_rad, height, div) => {
                Ok(self.build_cylin(name, lower_rad, upper_rad, height, div, color)?)
            }
            BodiesOfRevolution::AbstractModel(model) => Ok(Box::new(AbstractModel::new(model))),
            _ => Err(Box::new(errors::not_impl_error::NotImplError::new(
                name.as_str(),
            ))),
        }
    }
}

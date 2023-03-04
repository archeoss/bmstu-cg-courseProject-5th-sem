pub use crate::managers::visitor::Visitor;
use crate::models::frame_model::triangle::Triangle;
use crate::models::frame_model::{FrameFigure, FrameModel, Point};
use crate::models::model::Model;
use nalgebra::{Matrix4, Point3, Vector3};
// use cgmath::{Matrix4, Vector3};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
#[derive(Default)]
pub struct TransformManager
{
    to_transform: Vec<bool>,
}

impl TransformManager
{
    #[must_use]
    pub const fn new() -> Self
    {
        Self {
            to_transform: Vec::new(),
        }
    }

    pub fn expand(&mut self)
    {
        self.to_transform.push(false);
    }

    pub fn remove(&mut self, index: usize)
    {
        self.to_transform.remove(index);
    }

    pub fn to_transform(&mut self) -> &mut [bool]
    {
        self.to_transform.as_mut_slice()
    }

    pub fn move_model(
        &mut self,
        obj: &Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>,
        dx: f64,
        dy: f64,
        dz: f64,
    )
    {
        let transform = Matrix4::new_translation(&Vector3::new(dx, dy, dz));
        println!("Transform: {transform:?}");
        obj.borrow_mut().transform_first(transform);
    }

    pub fn rotate_model(
        &mut self,
        obj: &Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>,
        ox: f64,
        oy: f64,
        oz: f64,
    )
    {
        let center = obj.borrow().true_center();
        let center = Vector3::new(center.x(), center.y(), center.z());
        // let transform = Matrix4::from_translation(Vector3::new(
        //     -center.x(),
        //     -center.y(),
        //     -center.z(),
        // )) * Matrix4::from_angle_x(cgmath::Rad(ox))
        //     * Matrix4::from_angle_y(cgmath::Rad(oy))
        //     * Matrix4::from_angle_z(cgmath::Rad(oz))
        //     * Matrix4::from_translation(Vector3::new(
        //         center.x(),
        //         center.y(),
        //         center.z(),
        //     ));
        // let transform = Matrix4::from_angle_x(cgmath::Rad(ox));
        // let transform = transform * Matrix4::from_angle_y(cgmath::Rad(oy));
        // let transform = transform * Matrix4::from_angle_z(cgmath::Rad(oz));
        let transform = Matrix4::new_translation(&-center)
            * Matrix4::new_rotation(Vector3::new(
                ox.to_radians(),
                oy.to_radians(),
                oz.to_radians(),
            ))
            * Matrix4::new_translation(&center);
        obj.borrow_mut().transform_self(transform);
    }

    pub fn scale_model(
        &mut self,
        obj: &Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>,
        kx: f64,
        ky: f64,
        kz: f64,
    )
    {
        let transform = Matrix4::new_nonuniform_scaling(&Vector3::new(kx, ky, kz));

        obj.borrow_mut().transform_self(transform);
    }

    pub fn move_models(
        &mut self,
        models: &mut [Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>],
        mv: (f64, f64, f64),
    )
    {
        if models.is_empty() {
            return;
        }
        let transform = Matrix4::new_translation(&Vector3::new(mv.0, mv.1, mv.2));

        #[cfg(debug_assertions)]
        {
            println!("Transform: {transform:?}");
        }
        for model in models.iter_mut() {
            model.borrow_mut().transform_first(transform);
        }
    }

    pub fn rotate_models(
        &mut self,
        models: &mut [Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>],
        rot: (f64, f64, f64),
    )
    {
        if models.is_empty() {
            return;
        }

        let center = models[0].borrow().true_center();
        let center = Vector3::new(center.x(), center.y(), center.z());
        // let transform = Matrix4::from_translation(Vector3::new(
        //     -center.x(),
        //     -center.y(),
        //     -center.z(),
        // )) * Matrix4::from_angle_x(cgmath::Rad(rot.0))
        //     * Matrix4::from_angle_y(cgmath::Rad(rot.1))
        //     * Matrix4::from_angle_z(cgmath::Rad(rot.2))
        //     * Matrix4::from_translation(Vector3::new(
        //         center.x(),
        //         center.y(),
        //         center.z(),
        //     ));

        let transform = Matrix4::new_translation(&-center)
            * Matrix4::new_rotation(Vector3::new(rot.0, rot.1, rot.2))
            * Matrix4::new_translation(&center);
        for model in models.iter_mut() {
            model.borrow_mut().transform_self(transform);
        }
    }

    pub fn scale_models(
        &mut self,
        models: &mut [Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>],
        scale: (f64, f64, f64),
    )
    {
        if models.is_empty() {
            return;
        }
        let transform = Matrix4::new_nonuniform_scaling(&Vector3::new(scale.0, scale.1, scale.2));

        for model in models.iter_mut() {
            model.borrow_mut().transform_self(transform);
        }
    }

    pub fn subtract(mesh1: RefCell<FrameFigure>, mesh2: RefCell<FrameFigure>) -> FrameFigure
    {
        impl Eq for Point<f64> {}
        // Add all of the points from mesh1 to the hash table
        // Weird bug with hashmap

        // Create a hash table to keep track of which points are duplicated
        let (mesh1, mesh2) = (mesh1.borrow(), mesh2.borrow());
        let mut vertex_map: HashMap<Point<f64>, usize> = HashMap::new();
        let mut points: Vec<Point<f64>> = Vec::new();
        let mut normals = Vec::new();
        let mut triangles: Vec<Triangle> = Vec::new();
        for (i, vertex) in mesh1.points().iter().enumerate() {
            vertex_map.insert(*vertex, i);
            points.push(vertex.clone());
        }

        // Iterate over each triangle in mesh2 and subtract it from mesh1
        let mut clipped_triangles: Vec<Triangle> = Vec::new();
        for triangle in mesh2.triangles() {
            // Calculate the plane equation for the triangle
            let v0 = mesh2.points()[triangle.a()]; //.position();
            let v1 = mesh2.points()[triangle.b()]; //.position();
            let v2 = mesh2.points()[triangle.c()]; //.position();
            let mut n = (v1 - v0).cross(&(v2 - v0));
            n.normalize();
            let d = n.dot(&v0);

            // Clip mesh1 against the plane of the triangle
            for i in 0..mesh1.triangles().len() {
                let mut inside_count = 0;
                let mut outside_count = 0;
                let mut intersection_points: Vec<Point<f64>> = Vec::new();

                // Iterate over each vertex in the triangle
                for j in 0..3 {
                    let vertex = &mesh1.points()[mesh1.triangles()[i].abc()[j]];
                    let distance = n.dot(&vertex) - d;

                    if distance > 0.0 {
                        // The vertex is inside the clipping plane
                        inside_count += 1;
                    } else {
                        // The vertex is outside the clipping plane
                        outside_count += 1;
                    }

                    // Calculate the intersection point between the edge and the clipping plane
                    let next_vertex = &mesh1.points()[mesh1.triangles()[i].abc()[(j + 1) % 3]];
                    let next_distance = n.dot(&next_vertex) - d;
                    if (distance > 0.0 && next_distance < 0.0)
                        || (distance < 0.0 && next_distance > 0.0)
                    {
                        let t = distance / (distance - next_distance);
                        let intersection_point = vertex.lerp(&next_vertex, t);
                        intersection_points.push(intersection_point);
                    }
                }

                if inside_count == 3 {
                    // The entire triangle is inside the clipping plane
                    clipped_triangles.push(mesh1.triangles()[i].clone());
                } else if inside_count == 2 {
                    // One triangle is clipped into two triangles
                    let mut new_triangle = mesh1.triangles()[i].clone().abc();
                    for j in 0..3 {
                        if n.dot(&mesh1.points()[mesh1.triangles()[i].abc()[j]]) > d {
                            // The vertex is inside the clipping plane
                            new_triangle[j] = *vertex_map
                                .entry(mesh1.points()[mesh1.triangles()[i].abc()[j]])
                                .or_insert_with(|| {
                                    // Add a new vertex to the mesh
                                    let new_vertex = intersection_points.pop().unwrap();
                                    normals.push(mesh1.normals()[i]);
                                    points.push(new_vertex.clone());
                                    points.len() - 1
                                });
                        }
                    }
                    clipped_triangles.push(Triangle::new(&new_triangle));

                    let mut new_triangle = mesh1.triangles()[i].clone().abc();
                    for j in 0..3 {
                        if n.dot(&mesh1.points()[mesh1.triangles()[i].abc()[j]]) <= d {
                            // The vertex is outside the clipping plane
                            new_triangle[j] = *vertex_map
                                .entry(mesh1.points()[mesh1.triangles()[i].abc()[j]])
                                .or_insert_with(|| {
                                    // Add a new vertex to the mesh
                                    let new_vertex = intersection_points.pop().unwrap();
                                    normals.push(mesh1.normals()[i]);
                                    points.push(new_vertex.clone());
                                    points.len() - 1
                                });
                        }
                    }

                    clipped_triangles.push(Triangle::new(&new_triangle));
                } else if inside_count == 1 {
                    // One triangle is clipped into three triangles
                    let mut new_triangle = mesh1.triangles()[i].clone().abc();
                    for j in 0..3 {
                        if n.dot(&mesh1.points()[mesh1.triangles()[i].abc()[j]]) <= d {
                            // The vertex is outside the clipping plane
                            new_triangle[j] = *vertex_map
                                .entry(mesh1.points()[mesh1.triangles()[i].abc()[j]])
                                .or_insert_with(|| {
                                    // Add a new vertex to the mesh
                                    let new_vertex = intersection_points.pop().unwrap();
                                    normals.push(mesh1.normals()[i]);
                                    points.push(new_vertex.clone());
                                    points.len() - 1
                                });
                        }
                    }
                    clipped_triangles.push(Triangle::new(&new_triangle));

                    let mut new_triangle = mesh1.triangles()[i].clone().abc();
                    for j in 0..3 {
                        if n.dot(&mesh1.points()[mesh1.triangles()[i].abc()[j]]) > d {
                            // The vertex is inside the clipping plane
                            new_triangle[j] = *vertex_map
                                .entry((mesh1.points()[mesh1.triangles()[i].abc()[j]]))
                                .or_insert_with(|| {
                                    // Add a new vertex to the mesh
                                    let new_vertex = intersection_points.pop().unwrap();
                                    normals.push(mesh1.normals()[i]);
                                    points.push(new_vertex.clone());
                                    points.len() - 1
                                });
                        }
                    }
                    clipped_triangles.push(Triangle::new(&new_triangle));

                    let mut new_triangle = [0; 3];
                    for j in 0..3 {
                        if n.dot(&mesh1.points()[mesh1.triangles()[i].abc()[j]]) > d {
                            // The vertex is inside the clipping plane
                            new_triangle[j] = *vertex_map
                                .entry((mesh1.points()[mesh1.triangles()[i].abc()[j]]))
                                .or_insert_with(|| {
                                    // Add a new vertex to the mesh
                                    let new_vertex = intersection_points.pop().unwrap();
                                    normals.push(mesh1.normals()[i]);
                                    points.push(new_vertex.clone());
                                    points.len() - 1
                                });
                        }
                    }
                    clipped_triangles.push(Triangle::new(&new_triangle));
                }
                println!("{clipped_triangles:?}");
            }
        }
        // Create the final mesh from the clipped triangles
        for triangle in clipped_triangles {
            // Only add triangles that have not already been added
            if !triangles.contains(&triangle) {
                triangles.push(triangle);
            }
        }
        let mut fig = FrameFigure::new_with_points(points);
        fig.set_triangles(triangles);

        fig
    }
}

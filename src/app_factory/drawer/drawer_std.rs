use crate::app_factory::canvas_factory::canvas::Canvas;
use crate::app_factory::canvas_factory::Section;
use crate::app_factory::drawer::{Drawer, FrameDrawer};
use crate::models::frame_model::triangle::Triangle;
use crate::models::frame_model::{Edge, FrameModel, Point};
use crate::models::model::Model;
use crate::objects::camera::Camera;
use nalgebra::Point3;
use std::cell::RefCell;
use std::rc::Rc;
pub const NEGATIVE_Z_PROJECTION: f64 = -0.05;
pub const ZERO_BRIGHTNESS: f64 = 0.4;
pub const BRIGHTNESS_RANGE: f64 = 0.45;
pub struct DrawerSTD {
    canvas: Rc<RefCell<Box<dyn Canvas>>>,
    pov: Option<Rc<RefCell<Camera>>>,
    z_buffer: Vec<Vec<f64>>,
}
struct Face {
    edge1: usize,
    edge2: usize,
    edge3: usize,
}
impl DrawerSTD {
    pub fn new(canvas: Rc<RefCell<Box<dyn Canvas>>>) -> Self {
        Self {
            canvas,
            pov: None,
            z_buffer: vec![],
        }
    }

    fn bresenham(&mut self, mut start: Point3<i32>, end: Point3<i32>, color: [u8; 4]) {
        if !self.check_pos3_all([start, end].into_iter()) {
            return;
        }
        dbg!(start, end);
        let mut canvas = self.canvas.as_ref().borrow_mut();
        canvas.point(
            start.x, //.checked_div(start.z).unwrap_or(start.x),
            start.y, //.checked_div(start.z).unwrap_or(start.y),
            color, 1.0,
        );
        let dx = (end.x - start.x).abs();
        let dy = (end.y - start.y).abs();
        let dz = (end.z - start.z).abs();
        let xs = if end.x > start.x { 1 } else { -1 };
        let ys = if end.y > start.y { 1 } else { -1 };
        let zs = if end.z > start.z { 1 } else { -1 };
        // Driving axis is X-axis"
        if dx >= dy && dx >= dz {
            let mut p1 = 2 * dy - dx;
            let mut p2 = 2 * dz - dx;
            while start.x != end.x {
                start.x += xs;
                if p1 >= 0 {
                    start.y += ys;
                    p1 -= 2 * dx;
                }
                if p2 >= 0 {
                    start.z += zs;
                    p2 -= 2 * dx;
                }
                p1 += 2 * dy;
                p2 += 2 * dz;
                canvas.point(
                    start.x, //.checked_div(start.z).unwrap_or(start.x),
                    start.y, //.checked_div(start.z).unwrap_or(start.y),
                    color, 1.0,
                );
            }

        // Driving axis is Y-axis"
        } else if dy >= dx && dy >= dz {
            let mut p1 = 2 * dx - dy;
            let mut p2 = 2 * dz - dy;
            while start.y != end.y {
                start.y += ys;
                if p1 >= 0 {
                    start.x += xs;
                    p1 -= 2 * dy;
                }
                if p2 >= 0 {
                    start.z += zs;
                    p2 -= 2 * dy;
                }
                p1 += 2 * dx;
                p2 += 2 * dz;
                canvas.point(
                    start.x, //.checked_div(start.z).unwrap_or(start.x),
                    start.y, //.checked_div(start.z).unwrap_or(start.y),
                    color, 1.0,
                );
            }

        // Driving axis is Z-axis"
        } else {
            let mut p1 = 2 * dy - dz;
            let mut p2 = 2 * dx - dz;
            while start.z != end.z {
                start.z += zs;
                if p1 >= 0 {
                    start.y += ys;
                    p1 -= 2 * dz;
                }
                if p2 >= 0 {
                    start.x += xs;
                    p2 -= 2 * dz;
                }
                p1 += 2 * dy;
                p2 += 2 * dx;
                canvas.point(
                    start.x, //.checked_div(start.z).unwrap_or(start.x),
                    start.y, //.checked_div(start.z).unwrap_or(start.y),
                    color, 1.0,
                );
            }
        }
    }

    fn reset(&mut self) {
        self.z_buffer = vec![
            vec![f64::MAX; self.canvas.borrow().width() as usize];
            self.canvas.borrow().height() as usize
        ];
    }

    pub fn transform_and_add(
        &mut self,
        (points, normals, triangles): &(
            &[&Vec<Point<f64>>],
            &[&Vec<Point<f64>>],
            &[&Vec<Triangle>],
        ),
        light_source: Point<f64>,
        color: [u8; 4],
    ) {
        // for every triangle (3 points + 3 normal points):
        for ((p, n), t) in points.iter().zip(normals.iter()).zip(triangles.iter()) {
            for (triangle, normal) in t.iter().zip(n.iter()) {
                // transform new point
                let current_window = [p[triangle.a()], p[triangle.b()], p[triangle.c()]];
                //     transform_and_normalize(new_point, new_normal, matrix);
                // check if any part of triangle visible and triangle isn't rotated to background
                if self.check_pos_all(current_window.into_iter())
                    && self.check_normals_all([*normal].into_iter())
                {
                    // add transformed triangle polygon to buffer
                    self.add_polygon(current_window, normal, &light_source, color);
                }
            }
        }
    }

    fn add_polygon(
        &mut self,
        points: [Point<f64>; 3],
        mut normal: &Point<f64>,
        light_source: &Point<f64>,
        color: [u8; 4],
    ) {
        // cast Y coordinate to integer (coordinates of the screen are integers)
        // find brightnesses for all vertexes for furhter processing by Gouraud algorithm
        let brightnesses = self.find_brightnesses(points, normal, light_source);
        // divide triangle on 2 pairs of sections, which make up 2 triangles with
        // parallel to X axis edge
        // let points: Vec<Point<f64>> = points
        //     .iter()
        //     .map(|point| self.pov.as_ref().unwrap().borrow().project2(&point))
        //     .collect();
        let mut int_points: Vec<Point3<i64>> = points
            .iter()
            .map(|point| Point3::new(point.x() as i64, point.y() as i64, point.z() as i64))
            .collect();
        // sort points by Y coordinate
        self.sort_by_y(&mut int_points, normal);

        let sections = self.divide_on_sections(&int_points, brightnesses);
        self.process_sections(sections, color);
    }

    // application of Gouraud and Z-buffer algorithms for 2 processed triangles
    fn process_sections(&mut self, mut sections: [Section; 4], color: [u8; 4]) {
        let mut canvas_ref = self.canvas.borrow_mut();
        for pair in sections.chunks_mut(2) {
            if pair[0].x_start > pair[1].x_start {
                continue;
            }

            if pair[0].y_start < 0 {
                let diff = (-pair[0].y_start) as f64;
                for sec in pair.iter_mut() {
                    sec.x_start += diff * sec.x_step;
                    sec.br_start += diff * sec.br_step;
                    sec.z_start += diff * sec.z_step;
                }
                pair[0].y_start = 0;
            }
            let height = canvas_ref.height();
            let width = canvas_ref.width();
            for y in (pair[0].y_start..=pair[0].y_end)
                .filter(|&elem| elem < height as i64)
                .map(|y| y as usize)
            {
                let x_from = f64::round(pair[0].x_start) as usize;
                let x_to = f64::round(pair[1].x_start) as usize;
                let diff_x = (x_to - x_from) as f64;

                let mut br = pair[0].br_start;
                let br_diff = (pair[1].br_start - br) / diff_x;
                let mut z = pair[0].z_start;
                let z_diff = (pair[1].z_start - z) / diff_x;

                for x in (x_from..=x_to).filter(|&x| x < width as usize) {
                    if z < self.z_buffer[y][x] {
                        self.z_buffer[y][x] = z;
                        canvas_ref.point(x as i32, y as i32, color, br);
                    }

                    br += br_diff;
                    z += z_diff;
                }

                for sec in pair.iter_mut() {
                    sec.x_start += sec.x_step;
                    sec.br_start += sec.br_step;
                    sec.z_start += sec.z_step;
                }
            }
        }
    }

    fn sort_by_y(&mut self, int_points: &mut [Point3<i64>], normals: &Point<f64>) {
        for (&i, &j) in [0, 0, 1].iter().zip([2, 1, 2].iter()) {
            let condition = {
                let (a, b) = (&int_points[i], &int_points[j]);
                a.y > b.y || a.y == b.y && a.x > b.x
            };
            if condition {
                int_points.swap(i, j);
            }
        }
    }

    fn find_brightnesses(
        &mut self,
        points: [Point<f64>; 3],
        normal: &Point<f64>,
        light_source: &Point<f64>,
    ) -> [f64; 3] {
        let mut lsvs = Vec::with_capacity(3);
        for i in 0..3 {
            let mut lsv = points[i] - *light_source;
            lsv.normalize();
            lsvs.push(lsv);
        }
        [
            ZERO_BRIGHTNESS + BRIGHTNESS_RANGE * (normal.scalar_mul(&lsvs[0])),
            ZERO_BRIGHTNESS + BRIGHTNESS_RANGE * (normal.scalar_mul(&lsvs[1])),
            ZERO_BRIGHTNESS + BRIGHTNESS_RANGE * (normal.scalar_mul(&lsvs[2])),
        ]
    }

    fn divide_on_sections(
        &mut self,
        int_points: &[Point3<i64>],
        brightnesses: [f64; 3],
    ) -> [Section; 4] {
        if int_points[0].y == int_points[2].y {
            return [
                Section::new(
                    &int_points[0],
                    &int_points[2],
                    brightnesses[0],
                    brightnesses[2],
                ),
                Section::new(
                    &int_points[2],
                    &int_points[0],
                    brightnesses[2],
                    brightnesses[0],
                ),
                Section::new(
                    &int_points[2],
                    &int_points[0],
                    brightnesses[2],
                    brightnesses[0],
                ),
                Section::new(
                    &int_points[0],
                    &int_points[2],
                    brightnesses[0],
                    brightnesses[2],
                ),
            ];
        };

        let midpoint2 = self.find_midpoint2(&int_points[0], &int_points[2], int_points[1].y);
        let midbrightness = brightnesses[0]
            + (brightnesses[2] - brightnesses[0])
                * ((int_points[1].y - int_points[0].y) as f64
                    / (int_points[2].y - int_points[0].y) as f64);

        if midpoint2.x > int_points[1].x {
            [
                Section::new(
                    &int_points[0],
                    &int_points[1],
                    brightnesses[0],
                    brightnesses[1],
                ),
                Section::new(&int_points[0], &midpoint2, brightnesses[0], midbrightness),
                Section::new(
                    &int_points[1],
                    &int_points[2],
                    brightnesses[1],
                    brightnesses[2],
                ),
                Section::new(&midpoint2, &int_points[2], midbrightness, brightnesses[2]),
            ]
        } else {
            [
                Section::new(&int_points[0], &midpoint2, brightnesses[0], midbrightness),
                Section::new(
                    &int_points[0],
                    &int_points[1],
                    brightnesses[0],
                    brightnesses[1],
                ),
                Section::new(&midpoint2, &int_points[2], midbrightness, brightnesses[2]),
                Section::new(
                    &int_points[1],
                    &int_points[2],
                    brightnesses[1],
                    brightnesses[2],
                ),
            ]
        }
    }

    fn find_midpoint2(&mut self, min: &Point3<i64>, max: &Point3<i64>, mid_y: i64) -> Point3<i64> {
        let mult = if max.y == min.y {
            1.0
        } else {
            (mid_y - min.y) as f64 / (max.y - min.y) as f64
        };
        Point3::new(
            (min.x as f64 + (max.x - min.x) as f64 * mult) as i64,
            mid_y,
            (min.z as f64 + (max.z - min.z) as f64 * mult) as i64,
        )
    }

    fn check_pos3_all<Iter>(&mut self, mut points: Iter) -> bool
    where
        Iter: Iterator<Item = Point3<i32>> + Clone,
    {
        let (height, width) = (
            self.canvas.borrow().height() as i32,
            self.canvas.borrow().width() as i32,
        );
        let (near, far) = (
            self.pov.as_ref().unwrap().borrow().near() as i32,
            self.pov.as_ref().unwrap().borrow().far() as i32,
        );
        let all_left = points.clone().all(|p| p.x < 0);
        let all_right = points.clone().all(|p| p.x >= width);
        let all_down = points.clone().all(|p| p.y < 0);
        let all_up = points.all(|p| p.y >= height);
        !(all_left || all_right || all_down || all_up)
    }

    fn check_pos_all<Iter>(&mut self, mut points: Iter) -> bool
    where
        Iter: Iterator<Item = Point<f64>> + Clone,
    {
        let (height, width) = (self.canvas.borrow().height(), self.canvas.borrow().width());
        let (near, far) = (
            self.pov.as_ref().unwrap().borrow().near(),
            self.pov.as_ref().unwrap().borrow().far(),
        );
        let all_left = points.clone().all(|p| p.x() < 0.0);
        let all_right = points.clone().all(|p| p.x() >= width as f64);
        let all_down = points.clone().all(|p| p.y() < 0.0);
        let all_up = points.all(|p| p.y() >= height as f64);
        let all_far = points.all(|p| p.z() > far);
        let all_near = points.all(|p| p.z() < near);
        !(all_left || all_right || all_down || all_up || all_near || all_far)
    }

    fn check_normals_all<Iter>(&mut self, mut normals: Iter) -> bool
    where
        Iter: Iterator<Item = Point<f64>>,
    {
        if let Some(first) = normals.next() {
            let mut res = first.z() >= NEGATIVE_Z_PROJECTION;
            for norm in normals {
                res = res || norm.z() >= NEGATIVE_Z_PROJECTION;
            }
            res
        } else {
            false
        }
    }

    fn check_normals_all_sum<Iter>(&mut self, mut normals: Iter) -> bool
    where
        Iter: Iterator<Item = Point<f64>>,
    {
        if let Some(first) = normals.next() {
            let mut res = first;
            for norm in normals {
                res += norm;
            }
            res.z() > NEGATIVE_Z_PROJECTION
        } else {
            false
        }
    }

    pub fn compute_normals(points: &[Point<f64>], triangles: &[Triangle]) -> Vec<Point<f64>> {
        let mut normals = vec![];
        for triangle in triangles {
            let a = points[triangle.a()];
            let b = points[triangle.b()];
            let c = points[triangle.c()];

            let ab = b - a;
            let ac = c - a;
            let mut normal = Point::new(
                ab.y().mul_add(ac.z(), -ab.z() * ac.y()),
                ab.z().mul_add(ac.x(), -ab.x() * ac.z()),
                ab.x().mul_add(ac.y(), -ab.y() * ac.x()),
            );
            normal.normalize();
            normals.push(normal);
        }

        normals
    }

    // fn transform_and_normalize(
    //     &mut self,
    //     mut point: Point<f64>,
    //     mut norm_point: Point<f64>,
    //     matrix: &Matrix4<f64>,
    // ) -> (Point<f64>, Point<f64>)
    // {
    //     matrix.apply_to_point(&mut point);
    //     matrix.apply_to_point(&mut norm_point);
    //     let mut norm_vec = Vec3d::from_pts(&point, &norm_point);
    //     norm_vec.normalize();
    //
    //     (point, norm_vec)
    // }
}

impl Drawer for DrawerSTD {
    fn set_camera(&mut self, cam: Rc<RefCell<Camera>>) {
        self.pov = Some(cam);
    }

    fn set_canvas(&mut self, canvas: Rc<RefCell<Box<dyn Canvas>>>) {
        self.canvas = canvas;
    }

    fn draw_point(&mut self, x: i32, y: i32, color: [u8; 4]) {
        self.canvas.as_ref().borrow_mut().point(x, y, color, 1.0);
    }
    fn draw_line(&mut self, start: (i32, i32, i32), end: (i32, i32, i32), color: [u8; 4]) {
        self.bresenham(
            Point3::from_slice(unsafe {
                std::slice::from_raw_parts(std::ptr::addr_of!(start) as *const i32, 3)
            }),
            Point3::from_slice(unsafe {
                std::slice::from_raw_parts(std::ptr::addr_of!(end) as *const i32, 3)
            }),
            color,
        );
    }
    fn draw_line_aa(
        &mut self,
        _x_start: i32,
        _y_start: i32,
        _x_end: i32,
        _y_end: i32,
        _color: [u8; 4],
    ) {
        // self.wu(x_start, y_start, x_end, y_end, color);
    }

    fn draw_ellipse(&mut self, _x: i32, _y: i32, _width: i32, _height: i32, _color: [u8; 4]) {
        // self.canvas_factory.ellipse(x, y, width, height, color);
        // self.canvas_factory.wait_for_esc();
    }

    fn copy_to(&self, buffer: &mut [u8]) {
        buffer.copy_from_slice(self.canvas.as_ref().borrow().frame());
    }

    fn fill(&mut self, color: [u8; 4]) {
        self.canvas.as_ref().borrow_mut().fill(color);
    }

    fn frame(&self) -> Vec<u8> {
        self.canvas.as_ref().borrow().frame().to_vec()
    }
}

#[allow(clippy::cast_possible_truncation)]
impl FrameDrawer for DrawerSTD {
    fn draw_frame_model(
        &mut self,
        frame_models: &[Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>],
    ) {
        for frame_model in frame_models {
            let figure_rc = frame_model.as_ref().borrow();
            let height = self.canvas.as_ref().borrow_mut().height();
            let width = self.canvas.as_ref().borrow_mut().width();
            for figure in figure_rc.figures()[0].borrow().figures() {
                let pov = self.pov.as_ref().unwrap().borrow();
                let figure = figure.as_ref().borrow_mut();
                let tr = self.pov.as_ref().unwrap().borrow().view_projection();
                let center = Point::new(f64::from(width) / 2.0, f64::from(height) / 2.0, 0.0);
                let mut points: Vec<Point<f64>> = figure
                    .cached_points()
                    .iter()
                    .map(|p| center - pov.project(p))
                    .collect();
                let edges = figure.edges();
                drop(pov);
                for edge in edges {
                    let start = points[edge.from()];
                    let end = points[edge.to()];
                    self.draw_line(
                        (start.x() as i32, start.y() as i32, start.z() as i32),
                        (end.x() as i32, end.y() as i32, end.z() as i32),
                        [20, 230, 230, 230],
                    );
                }
            }
        }
    }
    fn draw_in_3d(
        &mut self,
        frame_models: &[Rc<RefCell<Box<dyn Model<Output = FrameModel>>>>],
        light: Point<f64>,
    ) {
        self.reset();
        let height = self.canvas.as_ref().borrow_mut().height();
        let width = self.canvas.as_ref().borrow_mut().width();
        let pov = self.pov.as_ref().unwrap().borrow();
        let center = Point::new(f64::from(width) / 2.0, f64::from(height) / 2.0, 0.0);
        let light = center - pov.project(&light);
        drop(pov);
        for frame_model in frame_models {
            frame_model.as_ref().borrow_mut().update_model();
            let figure_rc = frame_model.as_ref().borrow();
            for figure in figure_rc.figures()[0].borrow().figures() {
                let pov = self.pov.as_ref().unwrap().borrow();
                let figure = figure.borrow_mut();
                let mut points = figure.cached_points().clone();
                points = points.iter().map(|p| center - pov.project(p)).collect();
                let triangles = figure.triangles();
                let normals = Self::compute_normals(&points, triangles);
                drop(pov);
                self.transform_and_add(
                    &(&[&points], &[&normals], &[triangles]),
                    light,
                    figure.color(),
                );
            }
        }
    }
}

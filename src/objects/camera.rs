use std::f64::consts::PI;

use crate::macros::{getter, getter_setter, setter};
use crate::managers::transform_manager::Visitor;
use crate::models::frame_model::Point;
use crate::objects::object::Object;
use crate::objects::visibility::Visibility;
use nalgebra::base::Unit;
use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Vector3};
#[derive(Debug)]
pub struct Camera {
    pos: Point3<f64>,
    target: Point3<f64>,
    up: Vector3<f64>,
    projection: Matrix4<f64>,
    view: Matrix4<f64>,
    view_port: (f64, f64),
    fov: f64,
    near: f64,
    far: f64,
}

impl Camera {
    #[must_use]
    pub fn new(pos: Point<f64>, target: Point<f64>, up: Point<f64>) -> Self {
        let pos = Point3::new(pos.x(), pos.y(), pos.z());
        let target = Point3::new(target.x(), target.y(), target.z());
        let up = Vector3::new(up.x(), up.y(), up.z());
        Self {
            pos,
            target,
            up,
            projection: Matrix4::identity(),
            view: Matrix4::look_at_rh(&pos, &target, &up),
            view_port: (800.0, 600.0),
            fov: (PI / 40.0).to_radians(),
            near: 1.0,
            far: 1000.0,
        }
    }

    getter_setter!(
        pos: Point3<f64>,
        target: Point3<f64>,
        up: Vector3<f64>,
        projection: Matrix4<f64>,
        view: Matrix4<f64>,
        view_port: (f64, f64),
        fov: f64,
        near: f64,
        far: f64
    );

    #[must_use]
    pub fn projected_near(&self) -> f64 {
        self.project(&Point::new(self.pos.x, self.pos.y, self.near + self.pos.z))
            .z()
            * (self.pos.z - self.target.z).signum()
    }

    #[must_use]
    pub fn projected_far(&self) -> f64 {
        self.project(&Point::new(self.pos.x, self.pos.y, self.far + self.pos.z))
            .z()
            * (self.pos.z - self.target.z).signum()
    }

    pub fn look_at_target(&mut self) {
        let view = Matrix4::look_at_rh(&self.pos, &self.target, &self.up);
        self.projection = view * self.projection;
    }

    pub fn look_at(&mut self, pos: Point3<f64>, target: Point3<f64>, up: Vector3<f64>) {
        self.pos = pos;
        self.target = target;
        self.up = up;
        self.view = Matrix4::look_at_rh(&pos, &target, &up);
    }

    pub fn move_forward(&mut self, distance: f64) {
        let forward_vec = (self.target - self.pos).normalize();
        self.pos += distance * forward_vec;
        self.target += distance * forward_vec;
        // self.look_at_target();
    }

    pub fn move_right(&mut self, distance: f64) {
        let direction = self.up.cross(&(self.target - self.pos)).normalize();
        // dbg!(direction, self.pos, self.target);
        self.pos += direction * distance;
        self.target += direction * distance;
        // self.look_at_target();
    }

    pub fn move_up(&mut self, distance: f64) {
        let up_vec = self.up.normalize();
        self.pos += distance * up_vec;
        self.target += distance * up_vec;
        // self.look_at_target();
    }

    #[must_use]
    pub fn view_projection(&self) -> Matrix4<f64> {
        /* self.projection */
        Matrix4::look_at_rh(&self.pos, &self.target, &self.up)
    }

    #[must_use]
    pub fn project(&self, point: &Point<f64>) -> Point<f64> {
        let point = point.transform(&Matrix4::look_at_rh(
            &self.pos,
            // &Point3::new(point.x(), point.y(), point.z()),
            &self.target,
            &self.up,
        ));
        let z = point.z();
        // println!("rad: {}", (PI / 40.0).to_radians());
        // for some reason, Perspective3 fucks up z coordinates. I think i'm doing something wrong?
        // but this workaround works fine
        let mut point = point.transform(
            &Perspective3::new(
                self.view_port.0 / self.view_port.1,
                self.fov,
                self.near,
                self.far,
            )
            .to_homogeneous(),
        );
        point.set_z(z);

        point
    }
    pub fn pitch(&mut self, angle: f64) {
        // let right = Unit::new_normalize((self.target - self.pos).cross(&self.up));
        // // let forward = (self.target - self.pos).normalize();
        // // let up = self.up.normalize();
        // // let right = Unit::new_normalize(forward.cross(&up));
        // let pitch = Matrix4::from_axis_angle(&right, angle);
        // let new_target = pitch.transform_point(&self.target);
        // let new_up = pitch.transform_vector(&self.up);
        // self.target = Point3::from_homogeneous(new_target.to_homogeneous()).unwrap();
        // self.up = Vector3::from_homogeneous(new_up.to_homogeneous()).unwrap();
        self.update_view();
    }

    pub fn yaw(&mut self, angle: f64) {
        let tmp = self.target - self.pos;
        let forward = (self.target - self.pos).normalize();
        let up = self.up.normalize();
        let right = forward.cross(&up).normalize();
        let new_forward = forward * angle.cos() + right * angle.sin();
        self.target = self.pos + new_forward.normalize();
        self.update_view();
    }

    pub fn roll(&mut self, angle: f64) {
        let forward = Unit::new_normalize(self.target - self.pos);
        let roll = Matrix4::from_axis_angle(&forward, angle);
        let new_up = roll.transform_vector(&self.up).to_homogeneous();
        self.up = Vector3::from_homogeneous(new_up).unwrap();
        self.update_view();
    }

    fn update_view(&mut self) {
        self.view = Matrix4::look_at_rh(&self.pos, &self.target, &self.up);
    }
}

impl Object for Camera {
    fn add(&mut self, _obj: Box<dyn Object>) -> bool {
        false
    }

    fn remove(&mut self, _obj: Box<dyn Object>) -> bool {
        false
    }

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_camera(self);
    }

    fn transform_self(&mut self, transform: Matrix4<f64>) {
        // let vec = Vector4::new(self.pos.x(), self.pos.y(), self.pos.z(), 1.0);
        // let point: Vector4<f64> = transform * vec;
        // self.pos = Point::new(point.x, point.y, point.z);
        //
        // let vec = Vector4::new(
        //     self.target.x(),
        //     self.target.y(),
        //     self.target.z(),
        //     1.0,
        // );
        // let point: Vector4<f64> = transform * vec;
        // self.target = Point::new(point.x, point.y, point.z);
        //
        // let vec = Vector4::new(self.up.x(), self.up.y(), self.up.z(), 1.0);
        // let point: Vector4<f64> = transform * vec;
        // self.up = Point::new(point.x, point.y, point.z);
    }

    fn transform_first(&mut self, _transform: Matrix4<f64>) {
        // self.transform = transform * self.transform;
    }
}

impl Visibility for Camera {
    fn is_visible(&self) -> bool {
        false
    }
}

use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use crate::{models::frame_model::Point, objects::camera::Camera};

pub struct CameraManager {
    cameras: Vec<Rc<RefCell<Camera>>>,
    active_camera_index: usize,
}

impl Default for CameraManager {
    fn default() -> Self {
        Self::new(PI / 30.0, 1.0, 1000.0, (800.0, 600.0))
    }
}

impl CameraManager {
    #[must_use]
    pub fn new(init_fov: f64, init_near: f64, init_far: f64, view_port: (f64, f64)) -> Self {
        let mut man = Self {
            cameras: Vec::new(),
            active_camera_index: 0,
        };
        man.spawn_camera();
        man.update_fov(init_fov);
        man.update_near(init_near);
        man.update_far(init_far);
        man.update_aspect(view_port);

        man
    }

    pub fn spawn_camera(&mut self) -> bool {
        self.cameras.push(Rc::new(RefCell::new(Camera::new(
            Point::new(0.0, 0.0, 300.0),
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ))));

        true
    }

    pub fn spawn_camera_pos(
        &mut self,
        pos: (f64, f64, f64),
        target: (f64, f64, f64),
        up: (f64, f64, f64),
    ) -> bool {
        self.cameras.push(Rc::new(RefCell::new(Camera::new(
            Point::from(pos),
            Point::from(target),
            Point::from(up),
        ))));

        true
    }

    pub fn set_active_camera(&mut self, index: usize) {
        if index < self.cameras.len() {
            self.active_camera_index = index;
        } else {
            self.active_camera_index = 0;
            println!("invalid camera index");
        }
    }

    #[must_use]
    pub fn active_camera(&self) -> Option<Rc<RefCell<Camera>>> {
        self.cameras.get(self.active_camera_index).cloned()
    }

    #[must_use]
    pub fn camera(&self, index: usize) -> Option<Rc<RefCell<Camera>>> {
        self.cameras.get(index).cloned()
    }

    pub fn add_camera(&mut self, cam: Rc<RefCell<Camera>>) -> bool {
        self.cameras.push(cam);

        true
    }

    #[must_use]
    pub fn amount(&self) -> usize {
        self.cameras.len()
    }

    pub fn delete_camera(&mut self, id: usize) -> bool {
        if self.cameras.len() <= id {
            return false;
        }
        self.cameras.remove(id);
        if self.active_camera_index >= self.cameras.len() {
            self.set_active_camera(self.cameras.len() - 1);
        }
        true
    }

    pub fn pitch(&mut self, id: usize, grad: f64) -> bool {
        if self.cameras.len() <= id {
            return false;
        }
        self.cameras[id].borrow_mut().pitch(grad);

        true
    }

    pub fn yaw(&mut self, id: usize, grad: f64) -> bool {
        if self.cameras.len() <= id {
            return false;
        }
        self.cameras[id].borrow_mut().yaw(grad);

        true
    }

    pub fn roll(&mut self, id: usize, grad: f64) -> bool {
        if self.cameras.len() <= id {
            return false;
        }
        self.cameras[id].borrow_mut().roll(grad);

        true
    }

    pub fn update_far(&mut self, far: f64) {
        for camera in &self.cameras {
            camera.borrow_mut().set_far(far);
        }
    }

    pub fn update_near(&mut self, near: f64) {
        for camera in &self.cameras {
            camera.borrow_mut().set_near(near);
        }
    }

    pub fn update_aspect(&mut self, view_port: (f64, f64)) {
        for camera in &self.cameras {
            camera.borrow_mut().set_view_port(view_port);
        }
    }

    pub fn update_fov(&mut self, fov: f64) {
        for camera in &self.cameras {
            camera.borrow_mut().set_fov(fov);
        }
    }
}

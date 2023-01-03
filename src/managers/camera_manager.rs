use std::{cell::RefCell, rc::Rc};

use crate::{models::frame_model::Point, objects::camera::Camera};

#[derive(Default)]
pub struct CameraManager
{
    cameras: Vec<Rc<RefCell<Camera>>>,
}

impl CameraManager
{
    pub fn spawn_camera(&mut self) -> bool
    {
        self.cameras
            .push(Rc::new(RefCell::new(Camera::new(Point::new(
                0.0, 0.0, 0.0,
            )))));

        true
    }

    pub fn spawn_camera_pos(&mut self, pos: (f64, f64, f64)) -> bool
    {
        self.cameras
            .push(Rc::new(RefCell::new(Camera::new(Point::new(
                pos.0, pos.1, pos.2,
            )))));

        true
    }

    pub fn add_camera(&mut self, cam: Rc<RefCell<Camera>>) -> bool
    {
        self.cameras.push(cam);

        true
    }

    pub fn delete_camera(&mut self, id: usize) -> bool
    {
        if self.cameras.len() >= id {
            return false;
        }
        self.cameras.remove(id);

        true
    }
}

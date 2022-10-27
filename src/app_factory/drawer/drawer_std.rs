use std::cell::RefCell;
// pub mod canvas_factory;
use crate::app_factory::drawer::{Drawer, FrameDrawer};
use std::mem::swap;
use std::rc::Rc;
// use std::sync::{Arc, Mutex};
use crate::app_factory::canvas_factory::canvas::Canvas;
use crate::models::frame_model::{FrameFigure, Point};
use crate::models::model::Model;

pub struct DrawerSTD
{
    canvas: Rc<RefCell<Box<dyn Canvas>>>,
}

impl DrawerSTD
{
    pub fn new(canvas: Rc<RefCell<Box<dyn Canvas>>>) -> DrawerSTD
    {
        DrawerSTD { canvas }
    }

    fn bresenham(
        &mut self,
        mut x_start: i32,
        mut y_start: i32,
        mut x_end: i32,
        mut y_end: i32,
        color: [u8; 4],
    )
    {
        let mut canvas = self.canvas.as_ref().borrow_mut();
        let mut points = vec![];

        let is_steep = (y_end - y_start).abs() > (x_end - x_start).abs();
        if is_steep {
            swap(&mut x_start, &mut y_start);
            swap(&mut x_end, &mut y_end);
        }
        // let mut reversed = false;
        if x_start > x_end {
            swap(&mut x_start, &mut x_end);
            swap(&mut y_start, &mut y_end);
            // reversed = true;
        }
        let dx = x_end - x_start;
        let dy = (y_end - y_start).abs();
        let mut err = dx / 2;
        let mut y = y_start;
        let ystep: i32;
        if y_start < y_end {
            ystep = 1;
        } else {
            ystep = -1;
        }
        for x in x_start..(x_end + 1) {
            if is_steep {
                points.push((y, x));
            } else {
                points.push((x, y));
            }
            err -= dy;
            if err < 0 {
                y += ystep;
                err += dx;
            }
        }

        // if reversed {
        //     for i in 0..(points.len()/2) {
        //         let end = points.len()-1;
        //         points.swap(i, end-i);
        //     }
        // }
        // points
        for (x, y) in points {
            canvas.point(x, y, color);
        }
    }

    // fn wu(
    //     &mut self,
    //     mut x_start: i32,
    //     mut y_start: i32,
    //     mut x_end: i32,
    //     mut y_end: i32,
    //     color: [u8; 4],
    // )
    // {
    //     fn ipart(x: f32) -> i32
    //     {
    //         x.floor() as i32
    //     }
    //     fn fpart(x: f32) -> f32
    //     {
    //         x - x.floor()
    //     }
    //     fn rfpart(x: f32) -> f32
    //     {
    //         1.0 - fpart(x)
    //     }
    //     let steep = (x_end - x_start).abs() < (y_end - y_start).abs();
    //     if steep {
    //         swap(&mut x_start, &mut y_start);
    //         swap(&mut x_end, &mut y_end);
    //     }
    //     if x_start > x_end {
    //         swap(&mut x_start, &mut x_end);
    //         swap(&mut y_start, &mut y_end);
    //     }
    //     let dx = x_end - x_start;
    //     let dy = y_end - y_start;
    //     let mut gradient: f32 = 0.0;
    //     if dx != 0 {
    //         gradient = dy as f32 / dx as f32;
    //     }
    //
    //     let xend = x_start;
    //     let yend = y_start as f32 + gradient * ((xend - x_start) as f32);
    //     let xgap: f32 = rfpart(x_start as f32 + 0.5);
    //     let xpxl1 = xend;
    //     let ypxl1 = ipart(yend);
    //     // TODO: fix
    //     if steep {
    //         // self.canvas_factory.lock().unwrap().point(ypxl1, xpxl1, color[0..3] + (rfpart(yend) * xgap * I) as u8);
    //         // self.canvas_factory.lock().unwrap().point(ypxl1 + 1, xpxl1, color[0..3] + (fpart(yend) * xgap * I) as u8);
    //         // points.push(Point::new(ypxl1, xpxl1));
    //         // points.push(Point::new(ypxl1 + 1, xpxl1));
    //     } else {
    //         // self.canvas_factory.lock().unwrap().point(xpxl1, ypxl1, color[0..3] + (rfpart(yend) * xgap * I) as u8);
    //         // self.canvas_factory.lock().unwrap().point(xpxl1 + 1, ypxl1, color[0..3] + (fpart(yend) * xgap * I) as u8);
    //         // points.push(Point::new(xpxl1, ypxl1));
    //         // points.push(Point::new(xpxl1 + 1, ypxl1));
    //     }
    //
    //     let mut intery: f32 = yend + gradient;
    //     let xend = x_end;
    //     let yend = y_end as f32 + gradient * ((xend - x_start) as f32);
    //     let xgap: f32 = rfpart(x_start as f32 + 0.5);
    //     let xpxl2 = xend;
    //     let ypxl2 = ipart(yend);
    //     if steep {
    //         // self.canvas_factory.lock().unwrap().point(ypxl2, xpxl2, color[0..3] + (rfpart(yend) * xgap * I) as u8);
    //         // self.canvas_factory.lock().unwrap().point(ypxl2 + 1, xpxl2, color[0..3] + (fpart(yend) * xgap * I) as u8);
    //         // points.push(Point::new(ypxl2, xpxl2));
    //         // points.push(Point::new(ypxl2 + 1, xpxl2));
    //     } else {
    //         // self.canvas_factory.lock().unwrap().point(xpxl2, ypxl2, color[0..3] + (rfpart(yend) * xgap * I) as u8);
    //         // self.canvas_factory.lock().unwrap().point(xpxl2 + 1, ypxl2, color[0..3] + (fpart(yend) * xgap * I) as u8);
    //         // points.push(Point::new(xpxl2, ypxl2));
    //         // points.push(Point::new(xpxl2 + 1, ypxl2));
    //     }
    //
    //     if steep {
    //         for x in xpxl1 + 1..xpxl2 - 1 {
    //             // self.canvas_factory.lock().unwrap().point(ipart(intery), x, color[0..3] + (rfpart(intery) * I) as u8);
    //             // self.canvas_factory.lock().unwrap().point(ipart(intery) + 1, x, color[0..3] + (fpart(intery) * I) as u8);
    //             // points.push(Point::new(ipart(intery), x));
    //             // points.push(Point::new(ipart(intery) + 1, x));
    //             intery += gradient;
    //         }
    //     } else {
    //         for x in xpxl1 + 1..xpxl2 - 1 {
    //             // self.canvas_factory.lock().unwrap().point(x, ipart(intery), color[0..3] + (rfpart(intery) * I) as u8);
    //             // self.canvas_factory.lock().unwrap().point(x, ipart(intery) + 1, color[0..3] + (fpart(intery) * I) as u8);
    //             // points.push(Point::new(x, ipart(intery)));
    //             // points.push(Point::new(x, ipart(intery) + 1));
    //             intery += gradient;
    //         }
    //     }
    //     // self.canvas_factory.points(&mut points, color, 255);
    //     // self.canvas_factory.lock().unwrap().re.render();
    // }
}

impl Drawer for DrawerSTD
{
    fn set_canvas(&mut self, canvas: Rc<RefCell<Box<dyn Canvas>>>)
    {
        self.canvas = canvas;
    }

    fn draw_point(&mut self, x: i32, y: i32, color: [u8; 4])
    {
        self.canvas.as_ref().borrow_mut().point(x, y, color);
        // self.canvas_factory.wait_for_esc();
    }
    fn draw_line(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32, color: [u8; 4])
    {
        self.bresenham(x_start, y_start, x_end, y_end, color);
    }
    fn draw_line_aa(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32, color: [u8; 4])
    {
        // self.wu(x_start, y_start, x_end, y_end, color);
    }

    fn draw_ellipse(&mut self, x: i32, y: i32, width: i32, height: i32, color: [u8; 4])
    {
        // self.canvas_factory.ellipse(x, y, width, height, color);
        // self.canvas_factory.wait_for_esc();
    }

    fn copy_to(&self, buffer: &mut [u8])
    {
        buffer.copy_from_slice(self.canvas.as_ref().borrow().get_frame());
    }

    fn fill(&mut self, color: [u8; 4])
    {
        self.canvas.as_ref().borrow_mut().fill(color);
    }

    fn get_frame(&self) -> Vec<u8>
    {
        self.canvas.as_ref().borrow().get_frame().to_vec()
    }
}

impl FrameDrawer for DrawerSTD
{
    fn draw_frame_model(&mut self, frame_model: Rc<RefCell<Box<dyn Model<Output = FrameFigure>>>>)
    {
        let figure_rc = frame_model.as_ref().borrow().get_model();
        let figure = figure_rc.as_ref().borrow_mut();
        let tr = frame_model.as_ref().borrow().get_transform();
        let points = figure.get_points();
        let edges = figure.get_edges();

        let height = self.canvas.as_ref().borrow_mut().get_height();
        let width = self.canvas.as_ref().borrow_mut().get_width();
        let center = Point::new(width as f32 / 2.0, height as f32 / 2.0, 0.0);
        for i in 0..edges.len() {
            let edge = edges[i];
            let start = points[edge.from as usize];
            let end = points[edge.to as usize];
            let start = center - start.transform(&tr);
            let end = center - end.transform(&tr);
            self.draw_line(
                start.get_x() as i32,
                start.get_y() as i32,
                end.get_x() as i32,
                end.get_y() as i32,
                [0, 255, 255, 255],
            );
        }
    }
}

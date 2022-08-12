// pub mod canvas;
use std::mem::swap;
use std::sync::{Arc, Mutex};
use canvas::Canvas;
use crate::app_factory::canvas;
use crate::app_factory::drawer::Drawer;

pub struct DrawerSTD
{
    canvas: Arc<Mutex<Box<dyn Canvas>>>
}

impl DrawerSTD
{
    pub fn new(canvas: Arc<Mutex<Box<dyn Canvas>>>) -> DrawerSTD
    {
        DrawerSTD{ canvas }
    }

    fn bresenham(&mut self, mut x_start: i32, mut y_start: i32, mut x_end: i32, mut y_end: i32, color: [u8; 4])
    {
        let mut dx = x_end - x_start;
        let mut dy = y_end - y_start;

        let mut x_sign = if dx > 0 { 1 } else { -1 };
        let mut y_sign = if dy > 0 { 1 } else { -1 };
        dx = dx.abs();
        dy = dy.abs();
        let mut turned = false;

        fn swap(a: &mut i32, b: &mut i32)
        {
            let tmp = *a;
            *a = *b;
            *b = tmp;
        }

        if dx < dy
        {
            turned = true;
            swap(&mut x_start, &mut y_start);
            swap(&mut dx, &mut dy);
            swap(&mut x_sign, &mut y_sign);
        }
        let incr_a = -2 * dx;
        let incr_b = 2 * dy;
        let mut f = incr_b - dx;
        for i in 0..(dx + 1)
        {
            if turned
            {
                self.canvas.lock().unwrap().point(y_start, x_start, color);
            }
            else
            {
                self.canvas.lock().unwrap().point(x_start, y_start, color);
            }

            if f > 0
            {
                f += incr_a;
                y_start += y_sign;
            }
            if f < 0
            {
                f += incr_b;
                x_start += x_sign;
            }
        }
        // self.canvas.wait_for_esc();
    }

    fn wu(&mut self, mut x_start: i32, mut y_start: i32, mut x_end: i32, mut y_end: i32, color: [u8; 4])
    {
        fn ipart(x: f32) -> i32 {
            x.floor() as i32
        }
        fn fpart(x: f32) -> f32 {
            x - x.floor()
        }
        fn rfpart(x: f32) -> f32 {
            1.0 - fpart(x)
        }
        let I: f32 = 255.0;
        let steep = (x_end - x_start).abs() < (y_end - y_start).abs();
        if steep
        {
            swap(&mut x_start, &mut y_start);
            swap(&mut x_end, &mut y_end);
        }
        if x_start > x_end
        {
            swap(&mut x_start, &mut x_end);
            swap(&mut y_start, &mut y_end);
        }
        let dx = x_end - x_start;
        let dy = y_end - y_start;
        let mut gradient: f32 = 0.0;
        if dx != 0
        {
            gradient = dy as f32 / dx as f32;
        }

        let xend = x_start;
        let yend = y_start as f32 + gradient * ((xend - x_start) as f32);
        let xgap: f32 = rfpart(x_start as f32 + 0.5);
        let xpxl1 = xend;
        let ypxl1 = ipart(yend);
        // TODO: fix
        if steep
        {
            // self.canvas.lock().unwrap().point(ypxl1, xpxl1, color[0..3] + (rfpart(yend) * xgap * I) as u8);
            // self.canvas.lock().unwrap().point(ypxl1 + 1, xpxl1, color[0..3] + (fpart(yend) * xgap * I) as u8);
            // points.push(Point::new(ypxl1, xpxl1));
            // points.push(Point::new(ypxl1 + 1, xpxl1));
        }
        else
        {
            // self.canvas.lock().unwrap().point(xpxl1, ypxl1, color[0..3] + (rfpart(yend) * xgap * I) as u8);
            // self.canvas.lock().unwrap().point(xpxl1 + 1, ypxl1, color[0..3] + (fpart(yend) * xgap * I) as u8);
            // points.push(Point::new(xpxl1, ypxl1));
            // points.push(Point::new(xpxl1 + 1, ypxl1));
        }

        let mut intery: f32 = yend + gradient;
        let xend = x_end;
        let yend = y_end as f32 + gradient * ((xend - x_start) as f32);
        let xgap: f32 = rfpart(x_start as f32 + 0.5);
        let xpxl2 = xend;
        let ypxl2 = ipart(yend);
        if steep
        {
            // self.canvas.lock().unwrap().point(ypxl2, xpxl2, color[0..3] + (rfpart(yend) * xgap * I) as u8);
            // self.canvas.lock().unwrap().point(ypxl2 + 1, xpxl2, color[0..3] + (fpart(yend) * xgap * I) as u8);
            // points.push(Point::new(ypxl2, xpxl2));
            // points.push(Point::new(ypxl2 + 1, xpxl2));
        }
        else
        {
            // self.canvas.lock().unwrap().point(xpxl2, ypxl2, color[0..3] + (rfpart(yend) * xgap * I) as u8);
            // self.canvas.lock().unwrap().point(xpxl2 + 1, ypxl2, color[0..3] + (fpart(yend) * xgap * I) as u8);
            // points.push(Point::new(xpxl2, ypxl2));
            // points.push(Point::new(xpxl2 + 1, ypxl2));
        }

        if steep
        {
            for x in xpxl1 + 1..xpxl2 - 1
            {
                // self.canvas.lock().unwrap().point(ipart(intery), x, color[0..3] + (rfpart(intery) * I) as u8);
                // self.canvas.lock().unwrap().point(ipart(intery) + 1, x, color[0..3] + (fpart(intery) * I) as u8);
                // points.push(Point::new(ipart(intery), x));
                // points.push(Point::new(ipart(intery) + 1, x));
                intery += gradient;
            }
        }
        else
        {
            for x in xpxl1 + 1..xpxl2 - 1
            {
                // self.canvas.lock().unwrap().point(x, ipart(intery), color[0..3] + (rfpart(intery) * I) as u8);
                // self.canvas.lock().unwrap().point(x, ipart(intery) + 1, color[0..3] + (fpart(intery) * I) as u8);
                // points.push(Point::new(x, ipart(intery)));
                // points.push(Point::new(x, ipart(intery) + 1));
                intery += gradient;
            }
        }
        // self.canvas.points(&mut points, color, 255);
        // self.canvas.lock().unwrap().re.render();
    }
}

impl Drawer for DrawerSTD
{
    fn set_canvas(&mut self, canvas: Arc<Mutex<Box<dyn Canvas>>>)
    {
        self.canvas = canvas;
    }

    fn draw_point(&mut self, x: i32, y: i32, color: [u8; 4])
    {
        self.canvas.lock().unwrap().point(x, y, color);
        // self.canvas.wait_for_esc();
    }
    fn draw_line(&mut self, mut x_start: i32, mut y_start: i32, x_end: i32, y_end: i32, color: [u8; 4])
    {
        self.wu(x_start, y_start, x_end, y_end, color);
    }
    fn draw_line_AA(&mut self, mut x_start: i32, mut y_start: i32, x_end: i32, y_end: i32, color: [u8; 4])
    {
        self.wu(x_start, y_start, x_end, y_end, color);
    }
    fn draw_ellipse(&mut self, x: i32, y: i32, width: i32, height: i32, color: [u8; 4])
    {
        // self.canvas.ellipse(x, y, width, height, color);
        // self.canvas.wait_for_esc();
    }
}

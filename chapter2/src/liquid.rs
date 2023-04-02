extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::rvector::RVector;

pub struct Liquid {
    x: f32,
    y: f32,
    height: f32,
    width: f32,
    drag_coef: f32,
}

impl Liquid {
    pub fn new(x: f32, y: f32, width: f32, height: f32, drag_coef: f32) -> Self {
        Liquid {
            x, y, height, width, drag_coef
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 160));
        let rect = Rect::new(self.x as i32, self.y as i32, self.width as u32, self.height as u32);
        canvas.fill_rect(rect);
    }

    pub fn contains(&self, location: RVector) -> bool {
        println!("{:?}", location );
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;
        let b = (location.x > self.x && location.x < x2) && (location.y > self.y && location.y < y2);
        println!("{:?} vs x:{:?} x2:{:?} y:{:?} y2:{:?} -> {:?}", location, self.x, x2, self.y, y2, b);
        b
    }

    pub fn drag(&self, velocity: RVector) -> RVector {
        let speed = velocity.mag();
        let dragMagnitude = self.drag_coef * speed.powi(2);

        let mut drag = velocity.clone();
        drag.mult(-1.0);
        drag.normalize();
        drag.mult(dragMagnitude);
        println!("Drag is {:?}", drag);
        return drag;
    }

}
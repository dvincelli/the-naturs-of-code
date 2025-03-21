extern crate sdl2;

use std::error::Error;

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
            x,
            y,
            height,
            width,
            drag_coef,
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        canvas.set_draw_color(Color::RGB(0, 0, 160));
        let rect = Rect::new(
            self.x as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
        );
        canvas.fill_rect(rect)?;
        Ok(())
    }

    pub fn contains(&self, location: RVector) -> bool {
        println!("{:?}", location);
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;
        let b =
            (location.x > self.x && location.x < x2) && (location.y > self.y && location.y < y2);
        println!(
            "{:?} vs x:{:?} x2:{:?} y:{:?} y2:{:?} -> {:?}",
            location, self.x, x2, self.y, y2, b
        );
        b
    }

    pub fn drag(&self, velocity: RVector) -> RVector {
        let speed = velocity.mag();
        let dragMagnitude = self.drag_coef * speed.powi(2);

        let mut drag = velocity;
        drag.mult(-1.0);
        drag.normalize();
        drag.mult(dragMagnitude);
        println!("Drag is {:?}", drag);
        drag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_liquid() {
        let liquid = Liquid::new(10.0, 20.0, 30.0, 40.0, 0.5);
        assert_eq!(liquid.x, 10.0);
        assert_eq!(liquid.y, 20.0);
        assert_eq!(liquid.width, 30.0);
        assert_eq!(liquid.height, 40.0);
        assert_eq!(liquid.drag_coef, 0.5);
    }

    #[test]
    fn test_contains() {
        let liquid = Liquid::new(10.0, 20.0, 30.0, 40.0, 0.5);
        let location_inside = RVector {
            x: 15.0,
            y: 25.0,
            z: 0.0,
        };
        let location_outside = RVector {
            x: 5.0,
            y: 15.0,
            z: 0.0,
        };

        assert_eq!(liquid.contains(location_inside), true);
        assert_eq!(liquid.contains(location_outside), false);
    }
}

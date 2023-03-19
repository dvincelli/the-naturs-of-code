use rand::prelude::*;

use crate::rvector::RVector;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;


#[derive(Debug)]
pub struct Mover {
    pub location: RVector,
    pub velocity: RVector,
    pub acceleration: RVector,
    pub mass: f32,
}

impl Mover {
    pub fn new(mass: f32, x: f32, y: f32) -> Self {
        Mover {
            location: RVector::new2d(x, y),
            velocity: RVector::new2d(0.0, 0.0),
            acceleration: RVector::new2d(0.0, 0.0),
            mass: mass
        }
    }

    pub fn apply_force(&mut self, force: &RVector) {
        let mut force = force.clone();
        force.div(self.mass);
        self.acceleration.add(&force);
    }

    pub fn update(&mut self) {
        self.velocity.add(&self.acceleration);
        self.location.add(&self.velocity);
        self.acceleration.mult(0.0);
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) {
        canvas.ellipse(
            (self.location.x as i16).try_into().unwrap(),
            (self.location.y as i16).try_into().unwrap(),
            ((self.mass * 16.0) as i16).try_into().unwrap(),
            ((self.mass * 16.0) as i16).try_into().unwrap(),
            Color::RGB(255, 0, 0)
        );
    }

    pub fn check_edges(&mut self, width: f32, height: f32) {
        if self.location.x > width {
            self.location.x = width;
            self.velocity.x *= -1.0;
        } else if self.location.x < 0.0 {
            self.velocity.x *= -1.0;
            self.location.x = 0.0;
        }

        if self.location.y > height {
            self.location.y = height;
            self.velocity.y *= -1.0;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mover() {
        let mut m1 = Mover::new(rand::random(), rand::random());
        let mut m2 = Mover::new(rand::random(), rand::random());

        let wind = RVector::new2d(1.0, 0.0);

        m1.apply_force(&wind);
        m2.apply_force(&wind);

        assert_eq!(m1.acceleration.x, 0.1);
        assert_eq!(m1.acceleration.y, 0.0);

        assert_eq!(m2.acceleration.x, 0.1);
        assert_eq!(m2.acceleration.y, 0.0);
    }
}
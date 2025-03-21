use crate::rvector::RVector;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::error::Error;

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
            mass,
        }
    }

    pub fn apply_force(&mut self, force: &RVector) {
        let mut force = *force;
        force.div(self.mass);
        self.acceleration.add(&force);
    }

    pub fn update(&mut self) {
        self.velocity.add(&self.acceleration);
        self.location.add(&self.velocity);
        self.acceleration.mult(0.0);
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        canvas.ellipse(
            (self.location.x as i16).try_into().unwrap(),
            (self.location.y as i16).try_into().unwrap(),
            ((self.mass * 16.0) as i16).try_into().unwrap(),
            ((self.mass * 16.0) as i16).try_into().unwrap(),
            Color::RGB(255, 0, 0),
        )?;
        Ok(())
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
    fn test_mover_new() {
        let m = Mover::new(2.0, 10.0, 20.0);
        assert_eq!(m.mass, 2.0);
        assert_eq!(m.location.x, 10.0);
        assert_eq!(m.location.y, 20.0);
        assert_eq!(m.velocity.x, 0.0);
        assert_eq!(m.velocity.y, 0.0);
        assert_eq!(m.acceleration.x, 0.0);
        assert_eq!(m.acceleration.y, 0.0);
    }

    #[test]
    fn test_mover_apply_force() {
        let mut m = Mover::new(1.0, 0.0, 0.0);
        let force = RVector::new2d(0.5, 0.5);

        m.apply_force(&force);

        assert_eq!(m.acceleration.x, 0.5);
        assert_eq!(m.acceleration.y, 0.5);
    }

    #[test]
    fn test_mover_check_edges() {
        let mut m = Mover::new(1.0, 101.0, 101.0);

        let force = RVector::new2d(1.0, 2.0);

        m.apply_force(&force);
        m.update();

        m.check_edges(100.0, 100.0);

        assert_eq!(m.location.x, 100.0);
        assert_eq!(m.location.y, 100.0);
        assert_eq!(m.velocity.x, -1.0);
        assert_eq!(m.velocity.y, -2.0);
    }
}

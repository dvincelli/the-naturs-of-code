
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::{pixels::Color, render::Canvas};

use crate::mover::Mover;
use crate::rvector::RVector;

#[derive(Debug)]
pub struct Attractor {
    mass: f32,
    location: RVector,
    g_coef: f32,
}

impl Attractor {
    pub fn new(x: f32, y: f32, mass: f32) -> Self {
        Attractor {
            location: RVector::new2d(x, y),
            mass,
            g_coef: 1.0,
        }
    }

    pub fn display(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.ellipse(
            (self.location.x as i16).try_into().unwrap(),
            (self.location.y as i16).try_into().unwrap(),
            ((self.mass * 2.0) as i16).try_into().unwrap(),
            ((self.mass * 2.0) as i16).try_into().unwrap(),
            Color::RGB(255, 0, 0),
        )?;
        Ok(())
    }

    pub fn attract(&self, mover: &Mover) -> RVector {
        let mut force = self.location;
        force.sub(&mover.location);

        let distance = constrain(force.mag(), 5.0, 25.0);
        force.normalize();

        let strength = (self.mass * mover.mass * self.g_coef) / distance.powi(2);
        force.mult(strength);

        force
    }
}

fn constrain(v: f32, min: f32, max: f32) -> f32 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_attractor() {
        let attractor = Attractor::new(10.0, 20.0, 5.0);
        assert_eq!(attractor.mass, 5.0);
        assert_eq!(attractor.location.x, 10.0);
        assert_eq!(attractor.location.y, 20.0);
        assert_eq!(attractor.g_coef, 1.0);
    }
    
    #[test]
    fn test_constrain() {
        assert_eq!(constrain(15.0, 10.0, 20.0), 15.0);
        assert_eq!(constrain(5.0, 10.0, 20.0), 10.0);
        assert_eq!(constrain(25.0, 10.0, 20.0), 20.0);
    }
    
    #[test]
    fn test_attract() {
        let attractor = Attractor::new(0.0, 0.0, 10.0);
        let mover = Mover::new(5.0, 5.0, 1.0);
        let force = attractor.attract(&mover);
        
        assert_eq!(force.x, -1.885732);
        assert_eq!(force.y, -0.3771464);
    }
}

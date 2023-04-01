use std::borrow::Borrow;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{pixels::Color, render::Canvas};
use sdl2::video::Window;

use crate::mover::Mover;
use crate::rvector::RVector;

#[derive(Debug)]
pub struct Attractor {
    mass: f32,
    location: RVector,
    g_coef: f32
}

impl Attractor {
    pub fn new(x: f32, y: f32, mass: f32) -> Self {
        Attractor {
            location: RVector::new2d(x, y),
            mass,
            g_coef: 1.0
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
        let mut force = self.location.clone();
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
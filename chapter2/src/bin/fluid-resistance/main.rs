
use std::{cell::RefCell};

use the_naturs_of_code::{world::World, rvector::RVector, mover::Mover, liquid::Liquid};
use sdl2::{render::Canvas, pixels::Color};

struct WorldWithFluidResistance {
    movers: Vec<RefCell<Mover>>,
    liquid: Liquid,
    friction_coeff: f32,
    gravity_coeff: f32,
}

impl World for WorldWithFluidResistance {
    fn new() -> Self {
        let mut movers: Vec<RefCell<Mover>> = Vec::<RefCell<Mover>>::with_capacity(20);

        // TODO get dimensions from window size or take them as arguments
        let liquid = Liquid::new(0.0, 768.0/2.0, 1024.0, 768.0/2.0, 0.12);

        for i in 0..movers.capacity() {
            let mass = rand::random::<f32>() * 5.0 + 1.0;
            movers.push(
                RefCell::new(Mover::new(mass, 1024.0/20.0 * (i as f32) + mass/2.0, 0.0 + mass/2.0))
            );
        }

        WorldWithFluidResistance {
            movers: movers,
            liquid,
            gravity_coeff: 0.2,
            friction_coeff: 0.01
         }
    }

    fn display(&self, canvas: &mut Canvas<sdl2::video::Window>) -> () {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        self.liquid.display(canvas);

        for rc in self.movers.iter() {
            let m = rc.borrow();
            m.display(canvas).ok();
        }


        canvas.present();
    }

    fn setup(&self, canvas: &mut Canvas<sdl2::video::Window>) -> () {
        ()
    }

    fn update(&mut self, delta_time: u32) -> () {
        let (w, h) = self.size();
        for rc in self.movers.iter_mut() {
            let m = rc.get_mut();

            if self.liquid.contains(m.location) {
                let drag = self.liquid.drag(m.velocity);
                println!("{:?}", drag);
                m.apply_force(&drag);
            }

            let gravity = RVector::new2d(0.0, self.gravity_coeff * m.mass);
            m.apply_force(&gravity);

            let mut friction = m.velocity.clone();
            friction.mult(-1.0);
            friction.normalize();
            friction.mult(self.friction_coeff);
            m.apply_force(&friction);

            m.update();
            m.check_edges(w as f32, h as f32);
        }
    }
}

fn main() {
    let mut world : WorldWithFluidResistance = World::new();
    world.run();
}
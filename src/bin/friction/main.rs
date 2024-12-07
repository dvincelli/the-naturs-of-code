use std::cell::RefCell;

use sdl2::{pixels::Color, render::Canvas};
use the_naturs_of_code::{mover::Mover, rvector::RVector, world::World};

struct WorldWithFriction {
    movers: Vec<RefCell<Mover>>,
    wind: RVector,
    friction_coeff: f32,
    gravity_coeff: f32,
}

impl World for WorldWithFriction {
    fn new() -> Self {
        let mut movers: Vec<RefCell<Mover>> = Vec::<RefCell<Mover>>::with_capacity(20);
        let wind = RVector::new2d(0.01, 0.0);

        for _i in 0..movers.capacity() {
            movers.push(RefCell::new(Mover::new(
                rand::random::<f32>() * 5.0 + 1.0,
                0.0,
                0.0,
            )))
        }
        WorldWithFriction {
            movers,
            wind,
            gravity_coeff: 0.2,
            friction_coeff: 0.01,
        }
    }

    fn display(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for rc in self.movers.iter() {
            let m = rc.borrow();
            m.display(canvas).ok();
        }

        canvas.present();
    }

    fn setup(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        
    }

    fn update(&mut self, delta_time: u32) {
        let (w, h) = self.size();
        for rc in self.movers.iter_mut() {
            let m = rc.get_mut();
            m.apply_force(&self.wind);

            let gravity = RVector::new2d(0.0, self.gravity_coeff * m.mass);
            m.apply_force(&gravity);

            let mut friction = m.velocity;
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
    let mut world: WorldWithFriction = World::new();
    world.run();
}

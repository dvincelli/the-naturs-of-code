
use std::{cell::RefCell};

use the_naturs_of_code::{world::World, mover::Mover, attractor::Attractor};
use sdl2::{render::Canvas, pixels::Color};

struct WorldOfAttractors {
    movers: Vec<RefCell<Mover>>,
    attractor: Attractor,
}

impl World for WorldOfAttractors {
    fn new() -> Self {
        let mut movers: Vec<RefCell<Mover>> = Vec::<RefCell<Mover>>::with_capacity(1);

        for i in 0..movers.capacity() {
            let mass = 0.5;
            let x = 250.0;
            let y = 550.0;
            let mut mover = Mover::new(mass, x, y);
            mover.velocity.x = 1.0;

            movers.push(
                RefCell::new(mover)
            );
        }

        WorldOfAttractors {
            movers: movers,
            attractor: Attractor::new(1024.0/2.0, 768.0 / 2.0, 20.0)
         }
    }

    fn display(&self, canvas: &mut Canvas<sdl2::video::Window>) -> () {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for rc in self.movers.iter() {
            let m = rc.borrow();
            m.display(canvas).ok();
        }
        self.attractor.display(canvas).ok();

        canvas.present();
    }

    fn setup(&self, canvas: &mut Canvas<sdl2::video::Window>) -> () {
        ()
    }

    fn update(&mut self, delta_time: u32) -> () {
        let (w, h) = self.size();
        for rc in self.movers.iter_mut() {
            let m = rc.get_mut();

            let f = self.attractor.attract(m);
            println!("Attraction force: {:?}", f);
            m.apply_force(&f);

            m.update();
        }
    }
}

fn main() {
    let mut world : WorldOfAttractors = World::new();
    world.run();
}
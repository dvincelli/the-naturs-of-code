extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use rand::prelude::*;

use the_naturs_of_code::rvector::RVector;
use the_naturs_of_code::mover::Mover;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Example 2.1: Forces", WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    let mut movers : Vec<Mover> = Vec::<Mover>::with_capacity(20);

    let mut wind = RVector::new2d(0.01, 0.0);
    let gravity = RVector::new2d(0.0, 0.2);

    for i in 0..movers.capacity() {
        let x = (WIDTH as f32) * rand::random::<f32>();
        let y = (HEIGHT as f32) * rand::random::<f32>();
        movers.push(
            Mover::new(
                    rand::random::<f32>() * 5.0 + 1.0,
                    0.0,
                    0.0
                )
            )
    }

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        for m in &mut movers {
            m.apply_force(&wind);
            let gravity = RVector::new2d(0.0, 0.1 * m.mass);
            m.apply_force(&gravity);
            m.update();
            m.display(&mut canvas);
            m.check_edges((WIDTH as f32), (HEIGHT as f32));
        }

        if rand::random() {
            wind.set2d(rand::random::<f32>() * 0.02, 0.0);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
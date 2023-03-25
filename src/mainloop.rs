extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::mover::Mover;
use crate::rvector::RVector;

const TICKS_FOR_NEXT_FRAME: u32 = 1000/60; // 60 FPS

pub trait World {
    fn new() -> Self; 

    fn title(&self) -> &str {
        "The Naturs of Code"
    }

    fn size(&self) -> (u32, u32) {
        (1024, 768)
    }

    fn setup(&self, canvas: &mut Canvas<Window>) -> ();
    fn update(&mut self, delta_time: u32) -> ();
    fn display(&self, canvas: &mut Canvas<Window>) -> ();

    fn run(&mut self) {
        /* init */
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let (w, h) = self.size().clone();

        let window = video_subsystem
            .window(
                self.title(),
                w.try_into().unwrap(),
                h.try_into().unwrap(),
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
    
        let timer = sdl_context.timer().expect("TimerSubsystem is required");
        let mut event_pump = sdl_context.event_pump().unwrap();

        /* setup */
        self.setup(&mut canvas);

        /* main loop */
        'running: loop {
            let start_time = timer.ticks();
            println!("Start ticks {:?}", start_time);

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    _ => { println!("Received event {:?}", event)}
                }
            }

            // reset canvas
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            // update world
            self.update(TICKS_FOR_NEXT_FRAME);

            // draw world
            self.display(&mut canvas);

            // calibrate
            let end_time = timer.ticks();
            println!("End ticks {:?}", end_time);
            let delta_time = end_time - start_time;
            println!("Delta {:?}", delta_time);
            if TICKS_FOR_NEXT_FRAME > delta_time {
                let delay = TICKS_FOR_NEXT_FRAME - (delta_time);
                ::std::thread::sleep(Duration::from_millis(delay as u64));
            }
        }
    }
}


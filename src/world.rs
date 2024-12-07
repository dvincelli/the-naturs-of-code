extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{render::Canvas, video::Window};
use std::time::Duration;

const TICKS_FOR_NEXT_FRAME: u32 = 1000 / 60; // 60 FPS

/**
 * This module defines a trait `World` that represents a world in a game or simulation.
 * It provides methods for setting up the world, updating it, and displaying it on a canvas.
 * The `World` trait also includes a `run` method that runs the main loop of the world.
 */
pub trait World {
    fn new() -> Self;

    fn title(&self) -> &str {
        "The Naturs of Code"
    }

    fn size(&self) -> (u32, u32) {
        (1024, 768)
    }

    /// .
    fn setup(&self, canvas: &mut Canvas<Window>);
    /// .
    fn update(&mut self, delta_time: u32);
    /// .
    fn display(&self, canvas: &mut Canvas<Window>);

    fn run(&mut self) {
        /* init */
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let (w, h) = self.size();

        let window = video_subsystem
            .window(self.title(), w, h)
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

                    _ => {
                        println!("Received event {:?}", event)
                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::pixels::Color;

    struct TestWorld;

    impl World for TestWorld {
        fn new() -> Self {
            TestWorld
        }

        fn setup(&self, _canvas: &mut Canvas<Window>) {}

        fn update(&mut self, _delta_time: u32) {}

        fn display(&self, _canvas: &mut Canvas<Window>) {}
    }

    #[test]
    fn test_world_creation() {
        let world = TestWorld::new();
        assert_eq!(world.title(), "The Naturs of Code");
        assert_eq!(world.size(), (1024, 768));
    }

    // This one hangs
    //#[test]
    //fn test_run_method() {
    //    let mut world = TestWorld::new();
    //    //world.run(); // This will run the main loop, but we can't test it directly

    //    // Add assertions here to test specific behavior within the run method
    //}

    #[test]
    fn test_color_creation() {
        let color = Color::RGB(0, 0, 0);
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }
}

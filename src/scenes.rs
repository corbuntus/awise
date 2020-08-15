extern crate sdl2;

pub mod mainscene;

use super::resources;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;

pub trait Scene {
    fn done(&self) -> bool;
    fn finish(&mut self);

    fn on_keydown(&mut self, _key: Keycode) {}
    fn on_keyup(&mut self, _key: Keycode) {}
    fn on_mouse_button_down(&mut self, _pos: (i32, i32), _button: i32) {}
    fn on_mouse_button_up(&mut self, _pos: (i32, i32), _button: i32) {}
    fn on_mouse_motion(&mut self, _pos: (i32, i32)) {}
    fn on_quit(&mut self) {}

    fn handle_common_events(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.finish(),
                Event::MouseMotion { x, y, .. } => {
                    self.on_mouse_motion((x, y));
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.on_keydown(key);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        self.on_keyup(key);
                    }
                }
                _ => (),
            }
        }
    }

    fn paint(&mut self, canvas: &mut Canvas);

    fn run(&mut self, canvas: &mut Canvas, event_pump: &mut EventPump) {
        use std::thread;
        use std::time;
        'running: loop {
            let tick_begin = time::Instant::now();

            self.handle_common_events(event_pump);
            self.paint(canvas);

            if self.done() {
                break 'running;
            }

            let tick_end = time::Instant::now();
            let tick_left = tick_end - tick_begin;
            thread::sleep(tick_left);
        }
    }
}

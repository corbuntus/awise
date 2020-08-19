extern crate sdl2;

pub mod mainscene;

use super::components;
use super::prefabs;
use super::resources;
use super::Canvas;

use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::EventPump;

pub trait Scene {
    fn done(&self) -> bool;
    fn finish(&mut self);
    fn next_scene<'a>(&'a mut self) -> Option<&'a mut dyn Scene>;
    fn set_next_scene(&mut self, scene: Box<dyn Scene>);

    // FIXME: I don't know which path to take, the event based or the state based
    // one. For the gameplay part, the state is usually what you want, but for UI
    // related stuff the event based is much more natural. For now I'm keeping
    // both.
    fn on_keydown(&mut self, _key: Keycode) {}
    fn on_keyup(&mut self, _key: Keycode) {}

    // TODO: Use sdl2::mouse::MouseButton instead of i32.
    fn on_mouse_button_down(&mut self, _pos: (i32, i32), _button: i32) {}
    fn on_mouse_button_up(&mut self, _pos: (i32, i32), _button: i32) {}

    fn on_mouse_motion(&mut self, _pos: (i32, i32)) {}
    fn on_mouse_wheel(&mut self, _y: i32) {}
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
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    self.on_mouse_button_down((x, y), mouse_btn as i32);
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    self.on_mouse_button_up((x, y), mouse_btn as i32);
                }
                Event::MouseWheel { y, .. } => {
                    self.on_mouse_wheel(y);
                }
                _ => (),
            }
        }
    }

    fn update(&mut self, mouse_state: &MouseState, keyboard_state: &KeyboardState);
    fn paint(&self, canvas: &mut Canvas);

    fn run(&mut self, canvas: &mut Canvas, event_pump: &mut EventPump) {
        use std::thread;
        use std::time;

        let target_fps = 50;
        let millis_per_frame = time::Duration::from_millis(1_000 / target_fps);
        'running: loop {
            let tick_begin = time::Instant::now();

            self.handle_common_events(event_pump);
            self.update(&event_pump.mouse_state(), &event_pump.keyboard_state());
            self.paint(canvas);

            if self.done() {
                break 'running;
            }

            let tick_end = time::Instant::now();
            let tick_left = millis_per_frame.checked_sub(tick_end - tick_begin);
            if let Some(tick_left) = tick_left {
                thread::sleep(tick_left);
            }
        }
        self.on_quit();

        if let Some(next_scene) = self.next_scene() {
            next_scene.run(canvas, event_pump);
        }
    }
}

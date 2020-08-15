use super::Canvas;
use super::Keycode;

pub struct MainScene<'a> {
    texture_container: super::resources::TexturesContainer<'a>,
    done: bool,
}

impl<'a> MainScene<'a> {
    pub fn new(texture_container: super::resources::TexturesContainer) -> MainScene {
        MainScene {
            texture_container,
            done: false,
        }
    }
}

impl<'a> super::Scene for MainScene<'a> {
    fn finish(&mut self) {
        self.done = true;
    }

    fn done(&self) -> bool {
        self.done
    }

    fn on_keydown(&mut self, _key: Keycode) {
        // self.finish();
    }

    fn on_keyup(&mut self, key: Keycode) {
        match key {
            Keycode::Escape => self.finish(),
            _ => (),
        }
    }

    fn on_mouse_motion(&mut self, pos: (i32, i32)) {
        println!("Mouse moved to {:?}", pos);
    }

    fn paint(&mut self, canvas: &mut Canvas) {
        canvas.set_draw_color(super::Color::RGB(0, 0, 0));
        canvas.clear();

        canvas
            .copy(&self.texture_container.player, None, None)
            .unwrap();

        canvas.present();
    }
}

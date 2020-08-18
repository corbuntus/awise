use super::components;
use super::prefabs;
use super::Canvas;
use super::KeyboardState;
use super::Keycode;
use prefabs::Prefab;

use std::rc::Rc;

pub struct MainScene {
    tick_counter: u128,
    texture_container: super::resources::TexturesContainer,
    tileset: prefabs::Tileset,
    player: prefabs::Player,
    done: bool,
}

impl MainScene {
    pub fn new(texture_container: super::resources::TexturesContainer) -> MainScene {
        let tileset = prefabs::Tileset::new(texture_container.swamp_texture.clone());
        // TODO: We don't want to make a new slice for this.
        let player = prefabs::Player::new(components::AnimatedSprite::new(Rc::new([
            texture_container.player_idle_down.clone(),
        ])));
        MainScene {
            tick_counter: 0,
            texture_container,
            tileset,
            player,
            done: false,
        }
    }
}

impl super::Scene for MainScene {
    fn finish(&mut self) {
        self.done = true;
    }

    fn done(&self) -> bool {
        self.done
    }

    fn on_mouse_button_down(&mut self, pos: (i32, i32), _button: i32) {
        self.player.transform.x = pos.0;
        self.player.transform.y = pos.1;
    }

    fn on_keyup(&mut self, key: Keycode) {
        match key {
            Keycode::Escape => self.finish(),
            _ => (),
        }
    }

    // TODO: Implement a keymap instead of a KeyboardState.
    fn update(&mut self, keyboard_state: &KeyboardState) {
        self.tick_counter += 1;
        self.player
            .control(self.tick_counter, keyboard_state, &self.texture_container);
    }

    fn paint(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(super::Color::RGB(0, 0, 0));
        canvas.clear();
        self.tileset.paint_into(canvas);
        self.player.paint_into(canvas);
        canvas.present();
    }
}

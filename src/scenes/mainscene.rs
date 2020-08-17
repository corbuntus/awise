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
    player: prefabs::Player,
    done: bool,
}

impl MainScene {
    pub fn new(texture_container: super::resources::TexturesContainer) -> MainScene {
        // TODO: We don't want to make a new slice for this.
        let mut player = prefabs::Player::new(components::AnimatedSprite::new(Rc::new([
            texture_container.player_idle_down.clone(),
        ])));
        player.transform.x = 400;
        player.transform.y = 300;
        MainScene {
            tick_counter: 0,
            texture_container,
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

    fn on_keydown(&mut self, key: Keycode) {
        match key {
            Keycode::W => (),
            _ => (),
        }
    }

    fn on_keyup(&mut self, key: Keycode) {
        match key {
            Keycode::Escape => self.finish(),
            _ => (),
        }
    }

    fn update(&mut self, keyboard_state: &KeyboardState) {
        self.tick_counter += 1;
        self.player
            .control(self.tick_counter, keyboard_state, &self.texture_container);
    }

    fn paint(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(super::Color::RGB(0, 0, 0));
        canvas.clear();
        self.player.paint_into(canvas);
        canvas.present();
    }
}

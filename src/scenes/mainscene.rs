use super::components;
use super::prefabs;
use super::Canvas;
use super::KeyboardState;
use super::MouseState;
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

    // TODO: Implement a keymap instead of a KeyboardState.
    fn update(&mut self, mouse_state: &MouseState, keyboard_state: &KeyboardState) {
        self.tick_counter += 1;
        self.player
            .control(self.tick_counter, keyboard_state, &self.texture_container);

        if mouse_state.left() {
            self.player.transform.x = mouse_state.x();
            self.player.transform.y = mouse_state.y();
        }
    }

    fn paint(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(super::Color::RGB(0, 0, 0));
        canvas.clear();
        self.tileset.paint_into(canvas);
        self.player.paint_into(canvas);
        canvas.present();
    }
}

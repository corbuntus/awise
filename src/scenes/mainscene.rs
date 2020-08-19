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
    on_scene_change_range: prefabs::Region<prefabs::Player>,
    player: Rc<prefabs::Player>,
    done: bool,
}

impl MainScene {
    pub fn new(texture_container: super::resources::TexturesContainer) -> MainScene {
        let tileset = prefabs::Tileset::new(texture_container.swamp_texture.clone());
        // TODO: We don't want to make a new slice for this.
        let player = Rc::new(prefabs::Player::new(components::AnimatedSprite::new(
            Rc::new([texture_container.player_idle_down.clone()]),
        )));

        let on_scene_change_range = prefabs::Region::new(400, 400, 200, 200, player.clone());

        MainScene {
            tick_counter: 0,
            texture_container,
            tileset,
            on_scene_change_range,
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

        // FIXME: This crashes because Rust does not allow a mutable reference and a
        // immutable reference at the same time.
        Rc::get_mut(&mut self.player).unwrap().control(
            self.tick_counter,
            keyboard_state,
            &self.texture_container,
        );

        self.on_scene_change_range.update();

        if mouse_state.left() {
            let mut tf = &mut Rc::get_mut(&mut self.player).unwrap().transform;
            tf.x = mouse_state.x();
            tf.y = mouse_state.y();
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

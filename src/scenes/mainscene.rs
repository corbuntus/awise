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
    on_scene_change_range: prefabs::Region,
    player: prefabs::Player,

    done: bool,
    next_scene: Option<Box<dyn super::Scene>>,
}

impl MainScene {
    pub fn new(texture_container: super::resources::TexturesContainer) -> MainScene {
        println!("main scene loaded!");
        let tileset = prefabs::Tileset::new(texture_container.swamp_texture.clone());
        // TODO: We don't want to make a new slice for this.
        let player = prefabs::Player::new(components::AnimatedSprite::new(Rc::new([
            texture_container.player_idle_down.clone(),
        ])));

        let on_scene_change_range = prefabs::Region::new(450, 400, 200, 200);

        MainScene {
            tick_counter: 0,
            texture_container,
            tileset,
            on_scene_change_range,
            player,
            done: false,
            next_scene: None,
        }
    }
}

impl super::Scene for MainScene {
    fn finish(&mut self) {
        self.done = true;
    }

    fn next_scene<'a>(&'a mut self) -> Option<&'a mut dyn super::Scene> {
        // from https://stackoverflow.com/a/55866511
        self.next_scene
            .as_mut()
            .map(|scene| &mut **scene as &mut dyn super::Scene)
    }

    fn set_next_scene(&mut self, scene: Box<dyn super::Scene>) {
        self.next_scene = Some(scene);
    }

    fn done(&self) -> bool {
        self.done
    }

    // TODO: Implement a keymap instead of a KeyboardState.
    fn update(&mut self, mouse_state: &MouseState, keyboard_state: &KeyboardState) {
        self.tick_counter += 1;

        self.player
            .control(self.tick_counter, keyboard_state, &self.texture_container);

        if self.on_scene_change_range.check_target(&self.player) {
            self.set_next_scene(Box::new(MainScene::new(self.texture_container.clone())));
            self.finish();
        }

        if mouse_state.left() {
            self.player.transform.x = mouse_state.x();
            self.player.transform.y = mouse_state.y();
        }
    }

    fn paint(&self, canvas: &mut Canvas) {
        canvas.set_draw_color(super::Color::RGB(0, 0, 0));
        canvas.clear();
        self.tileset.paint_into(canvas);
        self.on_scene_change_range.paint_into(canvas);
        self.player.paint_into(canvas);
        canvas.present();
    }
}

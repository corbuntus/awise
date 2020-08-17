use super::sdl2;
use std::path::Path;
use std::rc::Rc;

use super::components::Sprite;
use sdl2::render::Texture;

type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub struct TexturesContainer {
    pub player_idle_down: Rc<Sprite>,
    pub player_idle_side: Rc<Sprite>,
    pub player_idle_up: Rc<Sprite>,

    pub player_moving_side: Rc<[Rc<Sprite>]>,
    pub player_moving_up: Rc<[Rc<Sprite>]>,
    pub player_moving_down: Rc<[Rc<Sprite>]>,
}

pub fn load_sprites(texture_creator: &TextureCreator) -> TexturesContainer {
    let player_idle_down = load_sprite("res/player/idle-down.bmp", texture_creator);
    let player_idle_side = load_sprite("res/player/idle-side.bmp", texture_creator);
    let player_idle_up = load_sprite("res/player/idle-up.bmp", texture_creator);

    let player_moving_down = Rc::new([
        load_sprite("res/player/moving-down_0.bmp", texture_creator),
        load_sprite("res/player/moving-down_1.bmp", texture_creator),
        load_sprite("res/player/moving-down_2.bmp", texture_creator),
        load_sprite("res/player/moving-down_3.bmp", texture_creator),
    ]);

    let player_moving_side = Rc::new([
        load_sprite("res/player/moving-side_0.bmp", texture_creator),
        load_sprite("res/player/moving-side_1.bmp", texture_creator),
        load_sprite("res/player/moving-side_2.bmp", texture_creator),
        load_sprite("res/player/moving-side_3.bmp", texture_creator),
    ]);

    let player_moving_up = Rc::new([
        load_sprite("res/player/moving-up_0.bmp", texture_creator),
        load_sprite("res/player/moving-up_1.bmp", texture_creator),
        load_sprite("res/player/moving-up_2.bmp", texture_creator),
        load_sprite("res/player/moving-up_3.bmp", texture_creator),
    ]);

    TexturesContainer {
        player_idle_down,
        player_idle_side,
        player_idle_up,
        player_moving_down,
        player_moving_side,
        player_moving_up,
    }
}

fn load_sprite(path: &str, texture_creator: &TextureCreator) -> Rc<Sprite> {
    Rc::new(Sprite::new(load_texture(path, texture_creator)))
}

fn load_texture<'s>(path: &str, texture_creator: &'s TextureCreator) -> Texture {
    let surface = sdl2::surface::Surface::load_bmp(Path::new(path)).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    texture
}

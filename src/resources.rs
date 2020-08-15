use super::sdl2;
use std::path::Path;
use std::rc::Rc;

use sdl2::render::Texture;

type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

pub struct TexturesContainer<'a> {
    pub player: Rc<Texture<'a>>,
}

pub fn load_sprites(texture_creator: &TextureCreator) -> TexturesContainer {
    let player_sprite = load_sprite("res/player.bmp", texture_creator);

    TexturesContainer {
        player: Rc::new(player_sprite),
    }
}

fn load_sprite<'s>(path: &str, texture_creator: &'s TextureCreator) -> Texture<'s> {
    let surface = sdl2::surface::Surface::load_bmp(Path::new(path)).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    texture
}

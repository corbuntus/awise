use super::components;
use super::Canvas;
use super::Prefab;
use std::rc::Rc;

pub struct Tileset {
    sprite: Rc<components::Sprite>,
    // tiles
}

impl Tileset {
    pub fn new(sprite: Rc<components::Sprite>) -> Self {
        Tileset {
            sprite
            // tiles
        }
    }
}

impl Prefab for Tileset {
    fn paint_into(&self, canvas: &mut Canvas) {
        canvas.copy(&self.sprite.texture, None, None).unwrap();
    }
}
